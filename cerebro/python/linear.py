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


def regress(percentage, cpus, device):
    NUM_FEATURES = 4
    X = torch.zeros((len(cpus), NUM_FEATURES)).to(device)
    y = torch.zeros(len(cpus), 1).to(device)
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
        X[index][0] = cpu.gen / gen_max
        X[index][1] = cpu.ix / ix_max
        X[index][2] = cpu.clock_speed_max / clock_speed_max_max
        X[index][2] = cpu.max_tdp() / max_tdp_max
        y[index][0] = cpu.passmark_single_thread / passmark_single_thread_max

    X_train, X_test, y_train, y_test = train_test_split(
        X, y, test_size=(1 - percentage / 100), random_state=42
    )

    W_target = torch.randn(NUM_FEATURES, 1).to(device)
    b_target = torch.randn(1).to(device)

    y_hat = X.mm(W_target) + b_target.item()

    fc = torch.nn.Linear(W_target.size(0), 1).to(device)

    for batch_idx in range(0, 30001):
        fc.zero_grad()

        output = torch.nn.functional.smooth_l1_loss(fc(X), y).to(device)
        loss = output.item()

        output.backward()

        for param in fc.parameters():
            param.data.add_(-0.1 * param.grad)

        if 0 == batch_idx % 10000:
            y_hat = fc(X_test)
            mse = (
                np.square(
                    (y_hat.cpu().detach().numpy() - y_test.cpu().detach().numpy())
                    * passmark_single_thread_max
                )
            ).mean()
            print(str(percentage) + "% " + str(batch_idx) + " MSE: " + str(mse))

    y_hat = fc(X_test)
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
    for percentage in [5, 10, 50, 70, 90]:
        mse = regress(percentage, cpus, device)
        mses[percentage] = mse
        print(str(percentage) + "% MSE: " + str(mse))

    print()
    print("Num cpus: " + str(len(cpus)))
    for (percentage, mse) in mses.items():
        print(str(percentage) + "% MSE: " + str(mse))
