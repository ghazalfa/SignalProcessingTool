pub mod mods;


use mods::DataType::RawData;
use mods::Math::math;
use mods::Math;
use mods::digital_filters::fir::{lowpass,highpass,bandpass,notch,moving_average};
use biquad::*;





 fn main() {
    
    let float1: RawData = RawData::FloatVec(vec![1.0,2.0,2.0,4.0]);
    let cutoff_freq: Hertz<f32> = 0.2.hz();
    let sample_rate: Hertz<f32> = 0.5.hz();


    let mut filter: lowpass::butterworth::biquad_butterworth = lowpass::butterworth::biquad_butterworth::new(cutoff_freq, sample_rate);

    let result: Vec<f32> = filter.process(float1);



}


        

    


    


