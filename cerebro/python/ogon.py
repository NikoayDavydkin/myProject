#!/usr/bin/env python3
import orjson
from catboost import CatBoostRegressor, Pool
import numpy as np
from sklearn.model_selection import train_test_split
import torch
from torch.autograd import Variable


class Cpu:
    def new(json):
        cpu = Cpu()
        cpu.id = json["id"]
        cpu.title = json["attributes"]["title"]
        cpu.sourceUrl = json["attributes"]["sourceUrl"]
        cpu.created = json["attributes"]["created"]
        cpu.gen = json["attributes"]["intelCoreIx"]["gen"]
        cpu.ix = json["attributes"]["intelCoreIx"]["ix"]
        cpu.num_cores = json["attributes"]["cpuNumCores"]
        cpu.clock_speed = json["attributes"]["cpuClockSpeed"]
        cpu.clock_speed_max = json["attributes"]["cpuClockSpeedMax"]
        cpu.tdp = json["attributes"]["cpuTdp"]
        cpu.tdp_up = json["attributes"]["cpuTdpUp"]
        cpu.tdp_down = json["attributes"]["cpuTdpDown"]
        cpu.base_power = json["attributes"]["cpuBasePower"]
        cpu.max_turbo_power = json["attributes"]["cpuMaxTurboPower"]
        cpu.passmark_single_thread = json["attributes"]["cpuPassmarkSingleThread"]
        cpu.passmark = json["attributes"]["cpuPassmark"]
        return cpu

    def max_tdp(self):
        if self.max_turbo_power is not None:
            return self.max_turbo_power
        elif self.tdp_up is not None:
            return self.tdp_up
        else:
            return self.tdp


class Model(torch.nn.Module):
    def __init__(self, num_gens):
        super().__init__()
        self.num_gens = num_gens
        self.gen_map = torch.nn.Parameter(torch.randn(num_gens))
        self.b = torch.nn.Parameter(torch.randn(()))
        self.c = torch.nn.Parameter(torch.randn(()))
        self.d = torch.nn.Parameter(torch.randn(()))

    def forward(self, x_gen, x_ix, x_max_clock_speed, x_max_tdp):
        return (
            torch.sum(torch.nn.functional.one_hot(x_gen, self.num_gens) * self.gen_map, 1)
            + self.c * x_max_clock_speed
            + self.d * x_max_tdp
        )


def regress(percentage, cpus, device):
    x_gen = torch.zeros((len(cpus)), dtype=torch.int64).to(device)
    x_ix = torch.zeros((len(cpus))).to(device)
    x_clock_speed_max = torch.zeros((len(cpus))).to(device)
    x_max_tdp = torch.zeros((len(cpus))).to(device)
    y = torch.zeros(len(cpus)).to(device)
    gen_max = 1
    ix_max = 1
    clock_speed_max_max = 1
    max_tdp_max = 1
    passmark_single_thread_max = 1
    for cpu in cpus:
        if cpu.gen > gen_max:
            gen_max = cpu.gen
        if cpu.ix > ix_max:
            ix_max = cpu.ix
        if cpu.clock_speed_max > clock_speed_max_max:
            clock_speed_max_max = cpu.clock_speed_max
        if cpu.max_tdp() > max_tdp_max:
            max_tdp_max = cpu.max_tdp()
        if cpu.passmark_single_thread > passmark_single_thread_max:
            passmark_single_thread_max = cpu.passmark_single_thread
    for index in range(0, len(cpus)):
        cpu = cpus[index]
        x_gen[index] = cpu.gen
        x_ix[index] = cpu.ix / ix_max
        x_clock_speed_max[index] = cpu.clock_speed_max / clock_speed_max_max
        x_max_tdp[index] = cpu.max_tdp() / max_tdp_max
        y[index] = cpu.passmark_single_thread / passmark_single_thread_max

    x_gen_train, x_gen_test, x_ix_train, x_ix_test, x_clock_speed_max_train, x_clock_speed_max_test, x_max_tdp_train, x_max_tdp_test, y_train, y_test = train_test_split(
        x_gen,
        x_ix,
        x_clock_speed_max,
        x_max_tdp,
        y,
        test_size=(1 - percentage / 100),
        random_state=42,
    )

    fc = Model(gen_max + 1).to(device)

    optimizer = torch.optim.SGD(fc.parameters(), lr=0.01, momentum=0.9)

    for batch_idx in range(0, 50001):
        optimizer.zero_grad()

        output = fc(x_gen_train, x_ix_train, x_clock_speed_max_train, x_max_tdp_train)

        loss = torch.nn.functional.smooth_l1_loss(output, y_train)
        loss.backward()

        optimizer.step()

        if 0 == batch_idx % 1000:
            y_hat = fc(x_gen_test, x_ix_test, x_clock_speed_max_test, x_max_tdp_test)
            mse = (
                np.square(
                    (y_hat.cpu().detach().numpy() - y_test.cpu().detach().numpy())
                    * passmark_single_thread_max
                )
            ).mean()
            y_hat_full = fc(x_gen, x_ix, x_clock_speed_max, x_max_tdp)
            mse_full = (
                np.square(
                    (y_hat_full.cpu().detach().numpy() - y.cpu().detach().numpy())
                    * passmark_single_thread_max
                )
            ).mean()
            print(str(percentage) + "% " + str(batch_idx) + " MSE: " + str(mse) + " Full MSE: " + str(mse_full))

    y_hat = fc(x_gen_test, x_ix_test, x_clock_speed_max_test, x_max_tdp_test)
    mse = (
        np.square(
            (y_hat.cpu().detach().numpy() - y_test.cpu().detach().numpy())
            * passmark_single_thread_max
        )
    ).mean()
    # for index in range(0, len(cpus)):
    #     print(str(cpus[index].passmark_single_thread) + " " + str(predict[index]))
    return mse


if __name__ == "__main__":
    device = torch.device("cuda:0" if torch.cuda.is_available() else "cpu")
    print("Starting PyTorch regression on " + str(device))
    file = open("data.json")
    json = orjson.loads(file.read())
    cpus = []
    for item in json["data"]["products"]:
        cpu = Cpu.new(item)
        if (
            cpu.passmark_single_thread is not None
            and cpu.clock_speed_max is not None
            and cpu.max_tdp() is not None
        ):
            cpus.append(cpu)
    # for cpu in cpus:
    #     print(cpu.id)
    #     print("\t" + cpu.title)

    mses = {}

    # for percentage in [5, 10, 20, 30, 40, 50, 60, 70, 80, 85, 90]:
    for percentage in [99]:
        mse = regress(percentage, cpus, device)
        mses[percentage] = mse
        print(str(percentage) + "% MSE: " + str(mse))

    print()
    print("Num cpus: " + str(len(cpus)))
    for (percentage, mse) in mses.items():
        print(str(percentage) + "% MSE: " + str(mse))
