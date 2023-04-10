use anyhow::Result;
use bonfire_core::attribute_value::CpuGroup;
use bonfire_core::attribute_value::IntelCoreIx;
use bonfire_ids::attribute_ids;
use hashbrown::HashMap;
use tch::nn::OptimizerConfig;
use tch::{nn, Device, Kind, Tensor};

use crate::graphql_schema::Cpu;
use crate::graphql_schema::Product;
use crate::tch_models::cpu_performance::CpuPerformanceModel;
use crate::tch_models::cpu_performance::CpuPerformanceModelBuilder;

pub fn convert_group(group: &CpuGroup) -> Option<CpuGroup> {
    if let CpuGroup::IntelCoreIx(ix) = group {
        Some(CpuGroup::IntelCoreIx(IntelCoreIx { gen: ix.gen, ix: 0 }))
    } else {
        None
    }
}

pub fn f64_slice_max(slice: &[f64]) -> f64 {
    slice.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
}

pub fn calculate_synthbench(products: &[Product]) -> Result<Vec<Cpu>> {
    let mut groups = HashMap::<CpuGroup, i64>::new();
    let mut cpus: Vec<Cpu> = products
        .iter()
        .map(Cpu::new)
        .filter(|cpu| {
            cpu.clock_speed_max.is_some()
                && cpu.cache.is_some()
                && cpu.created.is_some()
                && cpu.max_tdp().is_some()
                && cpu
                    .benchmarks
                    .get(&attribute_ids::CPU_PASSMARK_SINGLE_THREAD)
                    .is_some()
        })
        .filter(|cpu| {
            if let Some(group) = &cpu.group {
                if let Some(group) = convert_group(group) {
                    groups.insert(group, 0);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
        .collect();

    for (group_index, index) in groups.values_mut().enumerate() {
        *index = group_index as i64;
    }

    let group: Vec<i64> = cpus
        .iter()
        .map(|cpu| {
            *groups
                .get(&convert_group(cpu.group.as_ref().unwrap()).unwrap())
                .unwrap()
        })
        .collect();

    let clock_speed_max: Vec<f64> = cpus
        .iter()
        .map(|cpu| cpu.clock_speed_max.unwrap() as f64)
        .collect();

    let tdp: Vec<f64> = cpus
        .iter()
        .map(|cpu| cpu.max_tdp().unwrap() as f64)
        .collect();

    let passmark_single_thread: Vec<f64> = cpus
        .iter()
        .map(|cpu| {
            *cpu.benchmarks
                .get(&attribute_ids::CPU_PASSMARK_SINGLE_THREAD)
                .unwrap()
        })
        .collect();

    let clock_speed_max_max = f64_slice_max(&clock_speed_max);
    let tdp_max = f64_slice_max(&tdp);
    let passmark_single_thread_max = f64_slice_max(&passmark_single_thread);

    tch::maybe_init_cuda();
    println!("Cuda available: {}", tch::Cuda::is_available());
    println!("Cudnn available: {}", tch::Cuda::cudnn_is_available());
    let device = Device::cuda_if_available();
    println!("Device: {:?}", device);

    let group = Tensor::of_slice(&group).to_device(device);
    let clock_speed_max =
        Tensor::of_slice(&clock_speed_max).to_device(device) / clock_speed_max_max;
    let tdp = Tensor::of_slice(&tdp).to_device(device) / tdp_max;
    let y =
        Tensor::of_slice(&passmark_single_thread).to_device(device) / passmark_single_thread_max;

    //Train the neural network
    let vs = nn::VarStore::new(device);
    let model = CpuPerformanceModel::new(vs.root(), groups.len() as i64);
    //let mut opt = nn::Sgd::default().build(&vs, 0.2).unwrap();
    let mut opt = nn::Adam::default().build(&vs, 0.01).unwrap();
    let x = CpuPerformanceModelBuilder::builder()
        .group(&group)
        .clock_speed_max(&clock_speed_max)
        .tdp(&tdp)
        .build();
    for i in 0..30_001 {
        let loss = (model.forward(&x) - &y)
            .pow_tensor_scalar(2)
            .sum(Kind::Float)
            / cpus.len() as f64;
        opt.backward_step(&loss);
        if 0 == i % 1_000 {
            let mse = ((model.forward(&x) - &y) * passmark_single_thread_max)
                .pow_tensor_scalar(2)
                .sum(Kind::Float)
                / cpus.len() as f64;
            println!("{} MSE: {}", i, f64::from(mse));
        }
    }

    //Infer the synthetic benchmark
    let y_hat = Vec::<f64>::from(model.forward(&x) * passmark_single_thread_max);

    for (index, mut cpu) in cpus.iter_mut().enumerate() {
        let mut synth = HashMap::new();
        synth.insert(attribute_ids::CPU_PASSMARK_SINGLE_THREAD, y_hat[index]);
        cpu.synth = synth;
    }

    Ok(cpus)
}
