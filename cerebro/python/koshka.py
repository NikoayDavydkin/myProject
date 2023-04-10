#!/usr/bin/env python3
import orjson
from catboost import CatBoostRegressor, Pool
import numpy as np
from sklearn.model_selection import train_test_split


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


def regress(percentage, cpus):
    X = np.zeros((len(cpus), 4))
    y = np.zeros(len(cpus))
    for index in range(0, len(cpus)):
        cpu = cpus[index]
        X[index][0] = cpu.gen
        X[index][1] = cpu.ix
        X[index][2] = cpu.clock_speed_max
        X[index][2] = cpu.max_tdp()
        y[index] = cpu.passmark_single_thread

    X_train, X_test, y_train, y_test = train_test_split(
        X, y, test_size=(1 - percentage / 100), random_state=42
    )

    model = CatBoostRegressor(
        iterations=150, depth=8, learning_rate=1, loss_function="RMSE", verbose=True
    )
    # train the model
    model.fit(X_train, y_train)
    # make the prediction using the resulting model
    y_hat = model.predict(X_test)
    mse = (np.square(y_hat - y_test)).mean()
    # for index in range(0, len(cpus)):
    #     print(str(cpus[index].passmark_single_thread) + " " + str(predict[index]))
    return mse


if __name__ == "__main__":
    print("Loading Laptops from Ebay")
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
    for cpu in cpus:
        print(cpu.id)
        print("\t" + cpu.title)

    mses = {}

    for percentage in [5, 10, 20, 30, 40, 50, 60, 70, 80, 85, 90]:
        mse = regress(percentage, cpus)
        mses[percentage] = mse
        print(str(percentage) + "% MSE: " + str(mse))

    print()
    print("Num cpus: " + str(len(cpus)))
    for (percentage, mse) in mses.items():
        print(str(percentage) + "% MSE: " + str(mse))
