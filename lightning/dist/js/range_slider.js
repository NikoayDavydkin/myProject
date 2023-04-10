const screenSize = document.getElementById('screen_size');

if (screenSize){

noUiSlider.create(screenSize, {
    start: [0, 18],
    connect: true,
    step: 0.18,
    range: {
        'min': [0],
        'max': [18]
    }
});
}

let screen_size1 = document.getElementById('screen_size1');
let screen_size2 = document.getElementById('screen_size2');

let screen_size_arr = [screen_size1,screen_size2];

screenSize.noUiSlider.on('update', function(values,handle){
    screen_size_arr[handle].value = values[handle];
});





const screenResolution = document.getElementById('screen_resolution');

if (screenResolution){

noUiSlider.create(screenResolution, {
    start: [0,  11059200],
    connect: true,
    step: 110592,
    range: {
        'min': [0],
        'max': [11059200]
    }
});
}

const CPU_PassMark_Performance_Estimate = document.getElementById('cpu_passmark_performance_estimate');

if (CPU_PassMark_Performance_Estimate){

noUiSlider.create(CPU_PassMark_Performance_Estimate, {
    start: [0,  25410],
    connect: true,
    step: 254.1,
    range: {
        'min': [0],
        'max': [25410]
    }
});
}

const RAM_Size = document.getElementById('ram_size');

if (RAM_Size){

noUiSlider.create(RAM_Size, {
    start: [0,  256],
    connect: true,
    step: 2.56,
    range: {
        'min': [0],
        'max': [256]
    }
});
}

const SSD_Size = document.getElementById('ssd_size');

if (SSD_Size){

noUiSlider.create(SSD_Size, {
    start: [0,  8000],
    connect: true,
    step: 80,
    range: {
        'min': [0],
        'max': [8000]
    }
});
}

const HDD_Size = document.getElementById('hdd_size');

if (HDD_Size){

noUiSlider.create(HDD_Size, {
    start: [0,  1000],
    connect: true,
    step: 10,
    range: {
        'min': [0],
        'max': [1000]
    }
});
}

const GPU_PassMark_Performance_Estimate = document.getElementById('gpu_passmark_performance_estimate');

if (GPU_PassMark_Performance_Estimate){

noUiSlider.create(GPU_PassMark_Performance_Estimate, {
    start: [0,  16189],
    connect: true,
    step: 161.89,
    range: {
        'min': [0],
        'max': [16189]
    }
});
}

const price = document.getElementById('price');

if (price){

noUiSlider.create(price, {
    start: [0,  20000],
    connect: true,
    step: 200,
    range: {
        'min': [0],
        'max': [20000]
    }
});
}


