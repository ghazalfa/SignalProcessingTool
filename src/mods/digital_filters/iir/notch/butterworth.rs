
use biquad::*;
use crate::RawData;

///A struct representing a Butterworth notch filter implemented using the biquad algorithm.
///
///This struct is used to create and process a Butterworth bandpass filter on data provided
///in the form of `RawData` containing either `f32` or `i32` values.
///
/// #Issues
/// 
/// This filter does not align with Pythons scipy package so it is unreliable
pub struct BiquadButterworth {

    sample_rate: Hertz<f32>,       // The sample rate of the input data in Hz.
    high_cutoff: Hertz<f32>,       // The higher frequency cutoff of the notch filter in Hz.
    low_cutoff: Hertz<f32>,        // The lower frequency cutoff of the notch filter in Hz.
    biquad1: DirectForm1<f32>,     // The biquad filter implementation.

}

impl BiquadButterworth{


    /// Creates a new instance of the Butterworth notch filter.
    ///
    /// # Arguments
    /// * `high_cutoff` - The higher frequency cutoff of the notch filter in Hz.
    /// * `low_cutoff` - The lower frequency cutoff of the notch filter in Hz.
    /// * `sample_rate` - The sample rate of the input data in Hz.
    ///
    /// # Returns
    ///
    /// A new `biquad_butterworth` instance with the specified parameters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mods::digital_filters::iir:notch::butterworth::BiquadButterworth;
    /// use biquad::Hertz;
    ///
    /// let low_cutoff = 100.00.hz();
    /// let high_cutoff = 1000.0.hz()
    /// let sample_rate = 44100.0.hz();
    /// let butterworth_filter = BiquadButterworth::new(low_cutoff, high_cutoff, sample_rate);
    /// ```
    ///     
    pub fn new(high_cutoff: Hertz<f32>, low_cutoff: Hertz<f32>, sample_rate: Hertz<f32>) -> Self {

        //calculating centre frequency
        let centre_frequency: Hertz<f32> = ((low_cutoff.hz()+high_cutoff.hz())/2.0).hz(); 

        //calculating bandpass width
        let bandpass = high_cutoff.hz()-low_cutoff.hz();

        //calculating  q value
        let q: f32 = centre_frequency.hz()/bandpass;

       
        //creates coefficients and the filter        
        let coeffs = Coefficients::<f32>::from_params(Type::Notch, sample_rate, centre_frequency, q).unwrap();
        let biquad1: DirectForm1<f32> = DirectForm1::<f32>::new(coeffs);
        
        //returns an instance of the struct
        BiquadButterworth {
                    high_cutoff,
                    low_cutoff,
                    sample_rate,
                    biquad1,
                }

        }

    /// Processes the input data using the Butterworth notch filter and returns the filtered data.
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
    /// ```rust
    /// use mods::digital_filters::iir:notch::butterworth::BiquadButterworth;
    /// use crate::RawData;
    /// use biquad::Hertz;
    ///
    /// let low_cutoff = 100.00.hz();
    /// let high_cutoff = 1000.0.hz()
    /// let sample_rate = 44100.0.hz()
    /// let mut butterworth_filter = BiquadButterworth::new(low_cutoff, high_cutoff, sample_rate);
    ///
    /// let input_data = RawData::FloatVec(vec![0.1, 0.5, 0.8, 0.3, 0.6]);
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



    /// Applies the Butterworth notch filter using forward-backward filtering ("filtfilt").
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mods::digital_filters::iir:notch::butterworth::BiquadButterworth;
    /// use crate::RawData;
    /// use biquad::Hertz;
    ///
    /// let low_cutoff = 100.00.hz();
    /// let high_cutoff = 1000.0.hz()
    /// let sample_rate = 44100.0.hz();
    /// let mut butterworth_filter = BiquadButterworth::new(low_cutoff, high_cutoff, sample_rate);
    ///
    /// let input_data = RawData::FloatVec(vec![0.1, 0.5, 0.8, 0.3, 0.6]);
    /// let zero_phase_filtered_output = butterworth_filter.filtfilt(input_data);
    /// ```
    ///
    pub fn filtfilt(&mut self, input: RawData) -> Vec<f32>{

            //cloning the original filter
            let mut filterclone: BiquadButterworth = super::butterworth::BiquadButterworth::new(self.high_cutoff, self.low_cutoff, self.sample_rate);
            
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
