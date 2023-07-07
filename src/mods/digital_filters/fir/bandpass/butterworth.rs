
use biquad::*;
use crate::RawData;


//struct to implement the butterworth bandpass filter
pub struct biquad_butterworth {
    cutoff_freq: Hertz<f32>,
    sample_rate: Hertz<f32>,
    coeffs: Coefficients<f32>,
    biquad1: DirectForm1<f32>,

}

impl biquad_butterworth{

    //takes parameters in the Hz form from the biquad crate
    pub fn new(cutoff_freq: Hertz<f32>, sample_rate: Hertz<f32>) -> Self {

        //creates coefficients and the filter
        let coeffs = Coefficients::<f32>::from_params(Type::BandPass, sample_rate, cutoff_freq, Q_BUTTERWORTH_F32).unwrap();
        let mut biquad1: DirectForm1<f32> = DirectForm1::<f32>::new(coeffs);

        //returns an instance of the struct
        biquad_butterworth {
                    cutoff_freq,
                    sample_rate,
                    coeffs,
                    biquad1,
                }

        }

        //processes the data from the RawData struct and returns a vector of the same size containing the filtered data
        pub fn process(&mut self, input: RawData ) -> Vec<f32>{
            let input = input.clone_vector();
            let mut output_vec1: Vec<f32> = Vec::new();
            for elem in input {
                output_vec1.push(self.biquad1.run(elem));
            }

            return output_vec1;
        }

        //cant do function overloading in rust w/o making it more complicated w traits etc
        //thats why i went back and forth from Rawdata to Vec in the filtfilt function

        //need to make this more efficient
        pub fn filtfilt(&mut self, input: RawData) -> Vec<f32>{

            //cloning the original filter
            let mut filterclone: biquad_butterworth = super::butterworth::biquad_butterworth::new(self.cutoff_freq, self.sample_rate);
            
            //filtering the data
            let mut filtered_data: Vec<f32> = self.process(input);

            //reversing the data
            filtered_data.reverse();

            //changing data back to RawData format
            let filtered_data =  RawData::FloatVec(filtered_data);
            
            //filtering the reversed data
            let mut filtered_data = filterclone.process(filtered_data);

            //reversing the data
            filtered_data.reverse();

            return filtered_data;


        }


} 



