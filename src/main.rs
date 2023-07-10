pub mod mods;

use funspace::chebyshev::ortho::Chebyshev;
use ndarray::Array1;
use mods::datatype::RawData;
use mods::math::math;
use mods::digital_filters::fir::{lowpass,highpass,bandpass,notch,moving_average};
use mods::fast_fourier_transform;

use biquad::*;

 fn main() {
    
    // let float1: RawData = RawData::FloatVec(vec![1.0,2.0,2.0,4.0]);
    // let cutoff_freq: Hertz<f32> = 0.2.hz();
    // let sample_rate: Hertz<f32> = 0.5.hz();


    // let mut filter: lowpass::butterworth::biquad_butterworth = lowpass::butterworth::biquad_butterworth::new(cutoff_freq, sample_rate);

    // let result: Vec<f32> = filter.process(float1);


    // let mut filter = fast_fourier_transform::fft::new();


    // let input = RawData::FloatVec(vec![1.0,2.0,3.0]);


    // let output = filter.run_forward(&input);

    // print!("{:?}", output);


    let order = 4;  // Filter order
    let ripple = 0.1;  // Passband ripple factor
    let sample_rate = 44100.0;  // Sampling rate in Hz
    let cutoff_frequency = 5000.0;  // Cutoff frequency in Hz

    let b: Array1<f64> = cheby1_lowpass(order, ripple, sample_rate, cutoff_frequency);

    let input_data: Vec<f64> = // Your input data
    let output_data: Vec<f64> = input_data.iter().map(|&x| b.dot(&[x])).collect();


}


        

    


    


