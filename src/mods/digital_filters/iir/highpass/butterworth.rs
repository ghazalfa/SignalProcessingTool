
use biquad::*;
use crate::RawData;


///This struct is used to create and process a Butterworth highpass filter on data provided
///in the form of `RawData` containing either `f32` or `i32` values.
/// 
/// #Issues
/// 
/// Very minimal discrepencies from SciPy package in python
pub struct BiquadButterworth {

    cutoff_freq: Hertz<f32>,   // Frequency of the cutoff rate of filter in Hz
    sample_rate: Hertz<f32>,   // Frequency of the sampling rate in Hz
    biquad1: DirectForm1<f32>, // The biquad filter implementation

}

impl BiquadButterworth{

        /// Creates a new instance of the Butterworth highpass filter given the specified cutoff frequency.
        ///
        /// # Arguments
        ///
        /// * `cutoff_freq` - The frequency cutoff of the highpass filter in Hz.
        /// * `sample_rate` - The sample rate of the input data in Hz.
        ///
        /// # Returns
        ///
        /// A new `biquad_butterworth` instance with the specified parameters.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use mods::digital_filters::iir:highpass::butterworth::BiquadButterworth;
        /// use biquad::Hertz;
        ///
        /// let cutoff_freq = 100.00.hz();
        /// let sample_rate = 44100.0.hz();
        /// let butterworth_filter = BiquadButterworth::new(low_cutoff, high_cutoff, sample_rate);
        /// ```
        /// 
        pub fn new(cutoff_freq: Hertz<f32>, sample_rate: Hertz<f32>) -> Self {
             //Creates coefficients and the filter        
            let coeffs = Coefficients::<f32>::from_params(Type::HighPass, sample_rate, cutoff_freq, Q_BUTTERWORTH_F32).unwrap();
            let biquad1: DirectForm1<f32> = DirectForm1::<f32>::new(coeffs);
        
            //Returns an instance of the struct
            BiquadButterworth {
                    cutoff_freq,
                    sample_rate,
                    biquad1,
            }

        }

        
        //Processes the data from the RawData struct and returns a vector of the same size containing the filtered data
        ///
        /// # Arguments
        ///
        /// * `input` - The input data in `RawData` format (either `FloatVec` or `IntVec`).
        ///
        /// # Returns
        ///
        /// A vector of filtered `f32` data obtained from processing the input.
        ///
        /// # Examples
        ///
        /// ```
        /// use mods::digital_filters::iir::highpass::butterworth::BiquadButterworth;
        /// use crate::RawData;
        /// let cutoff_freq = 100.00.hz();
        /// let sample_rate = 44100.0.hz();
        ///
        /// let mut butterworth_filter = BiquadButterworth::new(cutoff_freq, sample_rate);
        /// let input_data = RawData::FloatVec(vec![0.5, 0.8, -0.2, 1.0, -0.7]);
        /// let filtered_output = butterworth_filter.process(input_data);
        /// ```
        /// 
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


        /// Applies the Butterworth highpass filter using forward-backward filtering ("filtfilt").
        ///
        /// This method applies the filter in both forward and backward directions to achieve zero-phase
        /// filtering, which reduces phase distortion.
        ///
        /// # Arguments
        ///
        /// * `input` - The input data in `RawData` format (either `FloatVec` or `IntVec`).
        ///
        /// # Returns
        ///
        /// A vector of filtered `f32` data obtained using the filtfilt process.
        /// # Examples
        ///
        /// ```
        /// use mods::digital_filters::iir::highpass::butterworth::BiquadButterworth;
        /// use crate::RawData;
        /// 
        /// let cutoff_freq = 100.00.hz();
        /// let sample_rate = 44100.0.hz();
        ///
        /// let mut butterworth_filter = BiquadButterworth::new(cutoff_freq,sample_rate);
        /// let input_data = RawData::FloatVec(vec![0.5, 0.8, -0.2, 1.0, -0.7]);
        /// let filtered_output = butterworth_filter.filtfilt(input_data);
        /// ```
        pub fn filtfilt(&mut self, input: RawData) -> Vec<f32>{

            //cloning the original filter
            let mut filterclone: BiquadButterworth = super::butterworth::BiquadButterworth::new(self.cutoff_freq, self.sample_rate);
            
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













