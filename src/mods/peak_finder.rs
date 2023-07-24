use crate::RawData;
use find_peaks::{self, Peak};



pub struct Peak_Finder{}

impl Peak_Finder{

    pub fn new() -> Self{
        Peak_Finder{}
    }


    //finds peaks in in the vector in the RawData struct, takes in no paramaters regarding height, prominence, distance, etc
    pub fn run_peakfinder(&mut self, input: &RawData) -> Vec<Peak<f32>>{

        //matches RawData struct to get to input
        match input{

            RawData::FloatVec(input) => {

                //calls on Peakfinder to find peaks without any parameters
                let peakvector: Vec<Peak<f32>> = find_peaks::PeakFinder::new(&input).find_peaks();

                return peakvector
            }

            RawData::IntVec(input) => {

                //converts the Vec<i32> to Vec<f32>
                let input: Vec<f32> = input.iter().map(|&x| x as f32).collect();

                //calls on Peakfinder to find peaks without any parameters                
                let peakvector: Vec<Peak<f32>> = find_peaks::PeakFinder::new(&input).find_peaks();

                return peakvector

            }
        }

    }


    //finds peaks in in the vector in the RawData struct, takes in parameters for minimum height and minimum prominence
    pub fn run_peakfinder_min_hp(&mut self, input: &RawData, min_height: f32, min_prominence: f32) -> Vec<Peak<f32>>{

        //matches RawData struct to get to input
        match input{

            RawData::FloatVec(input) => {

                //calls on Peakfinder to find peaks with a certain minimum height and prominence
                let peakvector: Vec<Peak<f32>> = find_peaks::PeakFinder::new(&input).with_min_height(min_height).with_min_prominence(min_prominence).find_peaks();

                return peakvector
            }

            RawData::IntVec(input) => {

                //converts the Vec<i32> to Vec<f32>
                let input: Vec<f32> = input.iter().map(|&x| x as f32).collect();

                //calls on Peakfinder to find peaks with a certain minimum height and prominence
                let peakvector: Vec<Peak<f32>> = find_peaks::PeakFinder::new(&input).with_min_height(min_height).with_min_prominence(min_prominence).find_peaks();

                return peakvector

            }
        }

    }

    //finds peaks in in the vector in the RawData struct, takes in parameters for minimum height and prominence, as well as max height and prominence
    pub fn run_peakfinder_min_hp_max_hp(&mut self, input: &RawData, min_height: f32, min_prominence: f32, max_height: f32, max_prominence: f32) -> Vec<Peak<f32>>{
        
        //matches RawData struct to get to input
        match input{

            RawData::FloatVec(input) => {

                //calls on Peakfinder to find peaks with a certain minimum height and prominence, and max height and prominence
                let peakvector: Vec<Peak<f32>> = find_peaks::PeakFinder::new(&input).with_min_height(min_height).with_min_prominence(min_prominence).with_max_height(max_height).with_max_prominence(max_prominence).find_peaks();

                return peakvector
            }

            RawData::IntVec(input) => {

                //converts the Vec<i32> to Vec<f32>
                let input: Vec<f32> = input.iter().map(|&x| x as f32).collect();

                //calls on Peakfinder to find peaks with a certain minimum height and prominence, and max height and prominence
                let peakvector: Vec<Peak<f32>> = find_peaks::PeakFinder::new(&input).with_min_height(min_height).with_min_prominence(min_prominence).with_max_height(max_height).with_max_prominence(max_prominence).find_peaks();

                return peakvector

            }
        }

    }

    //finds peaks in in the vector in the RawData struct, takes in parameters for minimum height, distance and plateau size
    pub fn run_peakfinder_min_hdp(&mut self, input: &RawData, min_height: f32, min_distance: f32, min_plateausize: f32) -> Vec<Peak<f32>>{
       
        //matches RawData struct to get to input
        match input{

            RawData::FloatVec(input) => {

                //calls on Peakfinder to find peaks with a certain minimum height, distance, and plateau size
                let peakvector: Vec<Peak<f32>> = find_peaks::PeakFinder::new(&input).with_min_height(min_height).with_min_distance(min_distance as usize).with_min_plateau_size(min_plateausize as usize).find_peaks();

                return peakvector
            }

            RawData::IntVec(input) => {

                //converts the Vec<i32> to Vec<f32>
                let input: Vec<f32> = input.iter().map(|&x| x as f32).collect();
                
                //calls on Peakfinder to find peaks with a certain minimum height, distance, and plateau size
                let peakvector: Vec<Peak<f32>> = find_peaks::PeakFinder::new(&input).with_min_height(min_height).with_min_distance(min_distance as usize).with_min_plateau_size(min_plateausize as usize).find_peaks();

                return peakvector

            }
        }

    }
}

