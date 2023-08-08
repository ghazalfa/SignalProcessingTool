use crate::RawData;
use sdr::fir::FIR;

pub struct FirBandpass{
    filter: FIR<f32>,
}

impl FirBandpass{

    pub fn new(n_taps: usize, cutoff1: f32, cutoff2:f32) -> Self  {

        FirBandpass {
           filter:  FIR::bandpass(n_taps,cutoff1 as f64,cutoff2 as f64)
         }

    }

    pub fn run_bandpass(&mut self, input: RawData) -> Vec<f32>{


        match input{
            
            
            RawData::FloatVec(input) => {


                let input: &[f32] = input.as_slice();

                let output: Vec<f32> = self.filter.process(input);

                return output;

            }

            RawData::IntVec(input) => {



                let input: Vec<f32> = input.iter().map(|&x| x as f32).collect();

                let input: &[f32] = input.as_slice();

                let output: Vec<f32> = self.filter.process(input);

                return output;

            }
        }
    }

    //need to make this more efficient
    pub fn filtfilt(&mut self, input: RawData) -> Vec<f32>{

        let n_taps = 5;
        let cutoff1 = 0.1;
        let cutoff2 = 0.4;

        //cloning the original filter
        let mut filterclone: FirBandpass = FirBandpass::new(n_taps, cutoff1, cutoff2);
        
        //filtering the data
        let mut filtered_data: Vec<f32> = self.run_bandpass(input);

        //reversing the data
        filtered_data.reverse();

        //changing data back to RawData format
        let filtered_data =  RawData::FloatVec(filtered_data);
        
        //filtering the reversed data
        let mut filtered_data = filterclone.run_bandpass(filtered_data);

        //reversing the data
        filtered_data.reverse();

        return filtered_data;


    }


}
