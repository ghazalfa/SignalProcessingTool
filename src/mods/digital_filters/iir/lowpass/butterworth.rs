
use biquad::*;
use crate::RawData;

///This struct is used to create and process a Butterworth lowpass filter on data provided
///in the form of `RawData` containing either `f32` or `i32` values.
/// 
/// #Issues
/// 
/// There is a known issue where the elements in the very beginning and very end of the signal are different than pythons
/// after being put through the filter
pub struct BiquadButterworth {

    cutoff_freq: Hertz<f32>,   // Frequency of the cutoff rate of filter in Hz
    sample_rate: Hertz<f32>,   // Frequency of the sampling rate in Hz
    biquad1: DirectForm1<f32>, // The biquad filter implementation

}

impl BiquadButterworth{

    
    /// Creates a new instance of `biquad_butterworth` with the specified cutoff frequency
    /// and sample rate in Hz
    /// 
    /// # Arguments
    ///
    /// * `cutoff_freq` - The  frequency cutoff of the highpass filter in Hz.
    /// * `sample_rate` - The sample rate of the input data in Hz.
    ///
    /// # Returns
    ///
    /// A new `biquad_butterworth` instance with the specified parameters.
    /// 
    /// # Examples
    ///
    /// ```
    /// use mods::digital_filters::iir::lowpass::butterworth::BiquadButterworth;
    /// use biquad::*;
    ///
    /// let cutoff_freq = 1000.00.hz()
    /// let sample_rate = 44100.hz()
    /// let butterworth_filter = BiquadButterworth::new(cutoff_freq, sample_rate);
    /// ```
    /// 
    pub fn new(cutoff_freq: Hertz<f32>, sample_rate: Hertz<f32>) -> Self {
        
        //creates coefficients and the filter            
        let coeffs: Coefficients<f32> = Coefficients::<f32>::from_params(Type::LowPass, sample_rate, cutoff_freq, Q_BUTTERWORTH_F32).unwrap();
        let biquad1: DirectForm1<f32> = DirectForm1::<f32>::new(coeffs);
        
        //returns an instance of the struct
        BiquadButterworth {
                    cutoff_freq,
                    sample_rate,
                    biquad1,
                }

        }
    
    /// Processes the input data using the Butterworth lowpass filter and returns the filtered data.
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
    /// use mods::digital_filters::iir:lowpass::butterworth::BiquadButterworth;
    /// use crate::RawData;
    /// use biquad::Hertz;
    ///
    /// let cutoff_freq = 1000.00.hz()
    /// let sample_rate = 44100.hz()
    /// let butterworth_filter = BiquadButterworth::new(cutoff_freq, sample_rate);
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


    /// Applies the Butterworth lowpass filter using forward-backward filtering ("filtfilt").
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
    /// use mods::digital_filters::iir:lowpass::butterworth::BiquadButterworth;
    /// use crate::RawData;
    /// use biquad::Hertz;
    ///
    /// let cutoff_freq = 1000.00.hz()
    /// let sample_rate = 44100.hz()
    /// let butterworth_filter = BiquadButterworth::new(cutoff_freq, sample_rate);
    ///
    /// let input_data = RawData::FloatVec(vec![0.1, 0.5, 0.8, 0.3, 0.6]);
    /// let zero_phase_filtered_output = butterworth_filter.filtfilt(input_data);
    /// ```
    ///    
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
















