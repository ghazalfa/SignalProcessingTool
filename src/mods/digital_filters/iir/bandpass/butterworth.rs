
use biquad::*;
use crate::RawData;


//struct to implement the butterworth bandpass filter
pub struct biquad_butterworth {

    sample_rate: Hertz<f32>,
    high_cutoff: Hertz<f32>,
    low_cutoff: Hertz<f32>,
    biquad1: DirectForm1<f32>,

}

impl biquad_butterworth{

    //takes parameters in the Hz form from the biquad crate
    pub fn new(low_cutoff: Hertz<f32>, high_cutoff: Hertz<f32>, sample_rate: Hertz<f32>) -> Self {

        let centre_frequency: Hertz<f32> = ((low_cutoff.hz()+high_cutoff.hz())/2.0).hz();
        let bandpass = high_cutoff.hz()-low_cutoff.hz();

        let q: f32 = centre_frequency.hz()/bandpass;

        //creates coefficients and the filter
        let coeffs = Coefficients::<f32>::from_params(Type::BandPass, sample_rate, centre_frequency, q).unwrap();
        let biquad1: DirectForm1<f32> = DirectForm1::<f32>::new(coeffs);

        //returns an instance of the struct
        biquad_butterworth {
                    sample_rate,
                    high_cutoff,
                    low_cutoff,
                    biquad1,
                }

        }

        //processes the data from the RawData struct and returns a vector of the same size containing the filtered data
        pub fn process(&mut self, input: RawData ) -> Vec<f32>{

            match input{

                RawData::FloatVec(input_vec) =>{

                    let mut output_vec1: Vec<f32> = Vec::with_capacity(input_vec.len());
    
                    output_vec1.extend(input_vec.into_iter().map(|elem| self.biquad1.run(elem)));
                
                    output_vec1
                    
                }

                RawData::IntVec(input_vec) =>{

                    let input_vec: Vec<f32> = input_vec.iter().map(|&x| x as f32).collect();


                    let mut output_vec1: Vec<f32> = Vec::with_capacity(input_vec.len());
    
                    output_vec1.extend(input_vec.into_iter().map(|elem| self.biquad1.run(elem)));
                
                    output_vec1

                }
            }

        }

        //cant do function overloading in rust w/o making it more complicated w traits etc
        //thats why i went back and forth from Rawdata to Vec in the filtfilt function

        //need to make this more efficient
        pub fn filtfilt(&mut self, input: RawData) -> Vec<f32>{

            //cloning the original filter
            let mut filterclone: biquad_butterworth = super::butterworth::biquad_butterworth::new(self.low_cutoff, self.high_cutoff, self.sample_rate);
            
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



