use crate::RawData;
use sdr::fir::FIR;

pub struct FirLowpass{
    filter: FIR<f32>,
}

impl FirLowpass{

    pub fn new(n_taps: usize, cutoff: f32) -> Self  {

         FirLowpass {
           filter:  FIR::lowpass(n_taps,cutoff as f64)
         }

    }

    pub fn run_lowpass(&mut self, input: RawData) -> Vec<f32>{


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
        let cutoff = 0.2;

        //cloning the original filter
        let mut filterclone: FirLowpass = FirLowpass::new(n_taps, cutoff);
        
        //filtering the data
        let mut filtered_data: Vec<f32> = self.run_lowpass(input);

        //reversing the data
        filtered_data.reverse();

        //changing data back to RawData format
        let filtered_data =  RawData::FloatVec(filtered_data);
        
        //filtering the reversed data
        let mut filtered_data = filterclone.run_lowpass(filtered_data);

        //reversing the data
        filtered_data.reverse();

        return filtered_data;


    }


}