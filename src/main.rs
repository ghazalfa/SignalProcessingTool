pub mod mods;

use std::vec;

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


    let input: Vec<f32> = vec![0.91836795, -0.7550365, -0.82229968, 0.38470112, 0.32756556, -0.1500332,
    -0.47942242, 0.78735185, -0.3130074, -0.78962053, 0.68406937, -0.10941988,
    0.85765562, -0.33443927, -0.88293069, -0.45943362, 0.7827355, -0.98156005,
    0.1983309, 1.0490953, 0.78269412, 0.48753985, -0.4683345, -0.08328862,
    0.47133147, 0.85514289, 0.02800922, 0.07371882, -0.28892876, 0.89355418,
    0.20687296, -0.98254069, 0.73257495, 0.97115261, 0.88601853, 0.49660099,
    -0.60699654, 0.99018712, 0.74853154, 1.03390956, 0.56113442, 0.93391565,
    -0.34803287, -0.49468972, 0.54351342, 0.54288017, 0.56071839, -0.6816979,
    0.44131403, 0.71416583, -0.20408451, -0.52847594, -0.86575061, 0.3189754,
    0.03358667, -0.62728735, -0.19742717, -0.08853031, 0.82239147, 0.38570716,
    -0.24534994, -0.55948792, 0.25647286, -0.60632081, 1.00364816, -0.40964857,
    0.84938726, -0.54761025, 0.86841654, -0.9697848, 0.81779373, -0.74209949,
    -0.01043969, 0.0437089, 0.91153038, 0.37510173, 0.6870717, -0.51927701,
    0.22990968, -0.11206606, -0.70855284, -0.68672557, 0.34639136, 0.07711553,
    0.74687533, -0.86588636, 0.60308048, 0.44784163, -0.48820096, -0.26549696,
    0.6144216, 0.44270719, 0.28133413, -0.79150201, 0.05373624, -0.59698319,
    -0.44591376, 0.16755127, 0.27518268, 0.40648236];

    let int_vector1 = RawData::FloatVec(input);



    let mut filter: fast_fourier_transform::fft = fast_fourier_transform::fft::new();

    let real_part: Vec<f32> = filter.run_forward_real(&int_vector1);
    let imag_part: Vec<f32> = filter.run_forward_imag(&int_vector1);


 
     // Print the real and imaginary parts
     println!("Real Part: {:?}", real_part);
     println!("Imaginary Part: {:?}", imag_part);



}


        

    


    


