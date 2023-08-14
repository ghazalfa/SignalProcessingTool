
use super::datatype::RawData;
use dsp::window;


/// The `Windowing` struct provides a set of windowing functions for segmenting and preprocessing raw data frames
/// before applying further signal processing or analysis. Windowing is a common technique used in various fields
/// such as audio processing, image analysis, and data science.
///
/// Each windowing function takes a data `frame` and a specified `size` or `increment`, applies a specific windowing
/// function to the frame, and returns a vector representing the windowed frame. This process helps reduce artifacts
/// and improve the quality of subsequent processing, such as spectral analysis or feature extraction.
///
/// Supported windowing functions include:
/// - Rectangular: Provides a basic windowing function with constant amplitude within the specified size.
/// - Blackman: Applies a Blackman window to the frame, tapering the edges to reduce spectral leakage.
/// - Hamming: Applies a Hamming window, which balances spectral leakage and side lobe attenuation.
/// - Hann: Uses a Hann window to improve spectral leakage and reduce side lobes compared to rectangular windows.
/// - Welch: Applies the Welch window, which reduces noise and provides better frequency domain representation.
/// - Sine: Uses a sine window to emphasize a particular frequency component in the frame.
/// - Triangular: Applies a triangular window for simple spectral analysis and smoothing.
///
/// Example Usage:
/// ```rust
/// let raw_data = RawData::FloatVec(vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6]);
/// let window_size = 4;
/// let windowed_data = Windowing::hann(&raw_data, window_size);
/// println!("Original Data: {:?}", raw_data);
/// println!("Windowed Data: {:?}", windowed_data);
/// ```
pub struct Windowing{}

impl Windowing{

    
    /// Applies windowing to a frame of data, splitting it into sub-vectors of a specified increment.
    ///
    /// # Parameters
    ///
    /// - `frame`: A reference to the input data of type `RawData`, which can be either `FloatVec` or `IntVec`.
    /// - `increment`: The increment value for splitting the frame into sub-vectors.
    ///
    /// # Returns
    ///
    /// A vector of sub-vectors, each representing a windowed portion of the input frame.
    pub fn windowing(frame: &RawData, increment: i32) -> Vec<Vec<f32>>{

        let vector: Vec<f32> = match frame{

            //to_vec.() works but i doubt its efficiency
            RawData::FloatVec(vector) => {vector.to_vec()}
            RawData::IntVec(vector) => {vector.iter().map(|&x| x as f32).collect()}
            
        };

                 let sub_vectors: Vec<Vec<f32>> = vector.chunks((increment as usize))
                                                 .map(|chunk| chunk.iter().map(|&x| x as f32).collect())
                                                 .collect();
                                            
        return sub_vectors;                                   
        
    
        // match frame{
        //     //if the vector is a float vector
        //     RawData::FloatVec(vector) => {

        //         let sub_vectors: Vec<Vec<f32>> = vector.chunks((increment as usize))
        //                                     .map(|chunk| chunk.to_vec())
        //                                     .collect();
                
        //         return sub_vectors;

        //     }

        //     //if the vector is type int
        //     RawData::IntVec(vector) => {

        //         let sub_vectors: Vec<Vec<f32>> = vector.chunks((increment as usize))
        //                                         .map(|chunk| chunk.iter().map(|&x| x as f32).collect())
        //                                         .collect();

        //         return sub_vectors;

        //     }

        // }
    }

    /// Applies a rectangular window of a specified size to a frame of data.
    ///
    /// # Parameters
    ///
    /// - `frame`: A reference to the input data of type `RawData`, which can be either `FloatVec` or `IntVec`.
    /// - `size`: The size of the rectangular window.
    ///
    /// # Returns
    ///
    /// A vector representing the windowed frame.
    pub fn rectangular(frame: &RawData, size: i32)-> Vec<f32>{


        match frame{
            //if the vector is a float vector
            RawData::FloatVec(frame) => {

                let win = window::rectangular(size as usize);
                let mut output = vec![0.0; size as usize];
                win.apply(&frame.as_slice(), &mut output);
                return output;



            }

            //if the vector is an int vector
            RawData::IntVec(frame) => {

                let win = window::rectangular(size as usize);
                let mut output = vec![0.0 ; size as usize];
                //converting frame to a from a Vec<>i32 to a [f32]
                let frame: &[f32] = &frame.iter().map(|&x| x as f32).collect::<Vec<f32>>()[..];
                win.apply(&frame, &mut output);
                return output;

                
            }


    }
}


    /// Applies a Blackman window of a specified size to a frame of data.
    ///
    /// # Parameters
    ///
    /// - `frame`: A reference to the input data of type `RawData`, which can be either `FloatVec` or `IntVec`.
    /// - `size`: The size of the Blackman window.
    ///
    /// # Returns
    ///
    /// A vector representing the windowed frame.
    pub fn blackman(frame: &RawData, size: i32) -> Vec<f32>{

        match frame{

            RawData::FloatVec(frame) => {
                
                let win = window::blackman(size as usize);
                let mut output = vec![0.0; size as usize];
                win.apply(&frame.as_slice(), &mut output);
                return output;

            }

            RawData::IntVec(frame) => {

                let win = window::blackman(size as usize);
                let mut output = vec![0.0 ; size as usize];
                //converting frame to a from a Vec<>i32 to a [f32]
                let frame: &[f32] = &frame.iter().map(|&x| x as f32).collect::<Vec<f32>>()[..];
                win.apply(&frame, &mut output);
                return output;


            }


        }

    }
    

    /// Applies a Hamming window of a specified size to a frame of data.
    ///
    /// # Parameters
    ///
    /// - `frame`: A reference to the input data of type `RawData`, which can be either `FloatVec` or `IntVec`.
    /// - `size`: The size of the Hamming window.
    ///
    /// # Returns
    ///
    /// A vector representing the windowed frame.
    pub fn hamming(frame: &RawData, size: i32) -> Vec<f32>{

        match frame{

            RawData::FloatVec(frame) => {
                
                let win = window::hamming(size as usize);
                let mut output = vec![0.0; size as usize];
                win.apply(&frame.as_slice(), &mut output);
                return output;

            }

            RawData::IntVec(frame) => {

                let win = window::hamming(size as usize);
                let mut output = vec![0.0 ; size as usize];
                //converting frame to a from a Vec<>i32 to a [f32]
                let frame: &[f32] = &frame.iter().map(|&x| x as f32).collect::<Vec<f32>>()[..];
                win.apply(&frame, &mut output);
                return output;


            }


        }

    }


    /// Applies a Hann window of a specified size to a frame of data.
    ///
    /// # Parameters
    ///
    /// - `frame`: A reference to the input data of type `RawData`, which can be either `FloatVec` or `IntVec`.
    /// - `size`: The size of the Hann window.
    ///
    /// # Returns
    ///
    /// A vector representing the windowed frame.
    pub fn hann(frame: &RawData, size: i32) -> Vec<f32>{

        match frame{

            RawData::FloatVec(frame) => {
                
                let win = window::hann(size as usize);
                let mut output = vec![0.0; size as usize];
                win.apply(&frame.as_slice(), &mut output);
                return output;

            }

            RawData::IntVec(frame) => {

                let win = window::hann(size as usize);
                let mut output = vec![0.0 ; size as usize];
                //converting frame to a from a Vec<>i32 to a [f32]
                let frame: &[f32] = &frame.iter().map(|&x| x as f32).collect::<Vec<f32>>()[..];
                win.apply(&frame, &mut output);
                return output;


            }


        }
    }

    /// Applies a Welch window of a specified size to a frame of data.
    ///
    /// # Parameters
    ///
    /// - `frame`: A reference to the input data of type `RawData`, which can be either `FloatVec` or `IntVec`.
    /// - `size`: The size of the Welch window.
    ///
    /// # Returns
    ///
    /// A vector representing the windowed frame.
    pub fn welch(frame: &RawData, size: i32) -> Vec<f32>{

        match frame{

            RawData::FloatVec(frame) => {
                
                let win = window::welch(size as usize);
                let mut output = vec![0.0; size as usize];
                win.apply(&frame.as_slice(), &mut output);
                return output;

            }

            RawData::IntVec(frame) => {

                let win = window::welch(size as usize);
                let mut output = vec![0.0 ; size as usize];
                //converting frame to a from a Vec<>i32 to a [f32]
                let frame: &[f32] = &frame.iter().map(|&x| x as f32).collect::<Vec<f32>>()[..];
                win.apply(&frame, &mut output);
                return output;


            }


        }

    }

    /// Applies a sine window of a specified size to a frame of data.
    ///
    /// # Parameters
    ///
    /// - `frame`: A reference to the input data of type `RawData`, which can be either `FloatVec` or `IntVec`.
    /// - `size`: The size of the sine window.
    ///
    /// # Returns
    ///
    /// A vector representing the windowed frame.
    pub fn sine(frame: &RawData, size: i32) -> Vec<f32>{

        match frame{

            RawData::FloatVec(frame) => {
                
                let win = window::sine(size as usize);
                let mut output = vec![0.0; size as usize];
                win.apply(&frame.as_slice(), &mut output);
                return output;

            }

            RawData::IntVec(frame) => {

                let win = window::sine(size as usize);
                let mut output = vec![0.0 ; size as usize];
                //converting frame to a from a Vec<>i32 to a [f32]
                let frame: &[f32] = &frame.iter().map(|&x| x as f32).collect::<Vec<f32>>()[..];
                win.apply(&frame, &mut output);
                return output;


            }


        }

    }
    
    /// Applies a triangular window of a specified size to a frame of data.
    ///
    /// # Parameters
    ///
    /// - `frame`: A reference to the input data of type `RawData`, which can be either `FloatVec` or `IntVec`.
    /// - `size`: The size of the triangular window.
    ///
    /// # Returns
    ///
    /// A vector representing the windowed frame.
    pub fn triangular(frame: &RawData, size: i32) -> Vec<f32>{

        match frame{

            RawData::FloatVec(frame) => {
                
                let win = window::triangular(size as usize);
                let mut output = vec![0.0; size as usize];
                win.apply(&frame.as_slice(), &mut output);
                return output;

            }

            RawData::IntVec(frame) => {

                let win = window::triangular(size as usize);
                let mut output = vec![0.0 ; size as usize];

                //converting frame to a from a Vec<>i32 to a [f32]
                let frame: &[f32] = &frame.iter().map(|&x| x as f32).collect::<Vec<f32>>()[..];
                win.apply(&frame, &mut output);
                return output;


            }


        }

    }

}

#[cfg(test)]
mod tests {

    use crate::RawData;
    use super::Windowing;
    use assert_approx_eq::assert_approx_eq;


    #[test]
    fn test_windowing(){

        let float_vector1 = RawData::FloatVec(vec![6.2,7.1,6.2]);
        let int_vector1 = RawData::IntVec(vec![1,2,3]);

        assert_eq!(Windowing::windowing(&float_vector1,2), vec![vec![6.2,7.1],vec![6.2]]);
        assert_eq!(Windowing::windowing(&int_vector1,2), vec![vec![1.0,2.0],vec![3.0]]);


    }

    #[test]
    fn test_rectangular(){

        let float_vector1 = RawData::FloatVec(vec![1.0;3]);
        let int_vector1 = RawData::IntVec(vec![1;3]);


        assert_eq!(Windowing::rectangular(&float_vector1, 3), vec![1.0, 1.0, 1.0]);
        assert_eq!(Windowing::rectangular(&int_vector1, 3), vec![1.0, 1.0, 1.0]);

    }

    #[test]
    fn test_blackman(){

        let float_vector1 = RawData::FloatVec(vec![1.0;5]);
        let int_vector1 = RawData::IntVec(vec![1;5]);

        let mut output = Windowing::blackman(&float_vector1, 5 );

        assert_approx_eq!(output[0], 0.00687, 1e-5f32);
        assert_approx_eq!(output[1], 0.34974, 1e-5f32);
        assert_approx_eq!(output[2], 1.0, 1e-5f32);
        assert_approx_eq!(output[3], 0.34974, 1e-5f32);
        assert_approx_eq!(output[4], 0.00687, 1e-5f32);

        output = Windowing::blackman(&int_vector1, 5 );

        assert_approx_eq!(output[0], 0.00687, 1e-5f32);
        assert_approx_eq!(output[1], 0.34974, 1e-5f32);
        assert_approx_eq!(output[2], 1.0, 1e-5f32);
        assert_approx_eq!(output[3], 0.34974, 1e-5f32);
        assert_approx_eq!(output[4], 0.00687, 1e-5f32);


    }
    
    #[test]   
    fn test_hamming(){

        let float_vector1 = RawData::FloatVec(vec![1.0;5]);
        let int_vector1 = RawData::IntVec(vec![1;5]);

        let mut output = Windowing::hamming(&float_vector1, 5 );

        assert_approx_eq!(output[0], 0.0869, 1e-3f32);
        assert_approx_eq!(output[1], 0.54347825, 1e-3f32);
        assert_approx_eq!(output[2], 1.0, 1e-3f32);
        assert_approx_eq!(output[3], 0.54347825, 1e-3f32);
        assert_approx_eq!(output[4], 0.0869, 1e-3f32);

        output = Windowing::hamming(&int_vector1, 5 );

        assert_approx_eq!(output[0], 0.0869, 1e-3f32);
        assert_approx_eq!(output[1], 0.54347825, 1e-3f32);
        assert_approx_eq!(output[2], 1.0, 1e-3f32);
        assert_approx_eq!(output[3], 0.54347825, 1e-3f32);
        assert_approx_eq!(output[4], 0.0869, 1e-3f32);


    }

    #[test]   
    fn test_hann(){

        let float_vector1 = RawData::FloatVec(vec![1.0;5]);
        let int_vector1 = RawData::IntVec(vec![1;5]);

        let mut output = Windowing::hann(&float_vector1, 5 );
        
        assert_approx_eq!(output[0], 0.0, 1e-5f32);
        assert_approx_eq!(output[1], 0.5, 1e-3f32);
        assert_approx_eq!(output[2], 1.0, 1e-3f32);
        assert_approx_eq!(output[3], 0.5, 1e-3f32);
        assert_approx_eq!(output[4], 0.0, 1e-5f32);

        output = Windowing::hann(&int_vector1, 5 );

        assert_approx_eq!(output[0], 0.0, 1e-5f32);
        assert_approx_eq!(output[1], 0.5, 1e-3f32);
        assert_approx_eq!(output[2], 1.0, 1e-3f32);
        assert_approx_eq!(output[3], 0.5, 1e-3f32);
        assert_approx_eq!(output[4], 0.0, 1e-5f32);


    }

    #[test]   
    fn test_welch(){

        let float_vector1 = RawData::FloatVec(vec![1.0;5]);
        let int_vector1 = RawData::IntVec(vec![1;5]);

        let mut output = Windowing::welch(&float_vector1, 5 );
        assert_eq!(output, vec![0.0, 0.75, 1.0, 0.75, 0.0]);

        output = Windowing::welch(&int_vector1, 5 );
        assert_eq!(output, vec![0.0, 0.75, 1.0, 0.75, 0.0]);

    }
   
    #[test]   
    fn test_sine(){

        let float_vector1 = RawData::FloatVec(vec![1.0;5]);
        let int_vector1 = RawData::IntVec(vec![1;5]);

        let mut output = Windowing::sine(&float_vector1, 5 );
        assert_approx_eq!(output[0], 0.0, 1e-5f32);
        assert_approx_eq!(output[1], 0.707, 1e-3f32);
        assert_approx_eq!(output[2], 1.0, 1e-3f32);
        assert_approx_eq!(output[3], 0.707, 1e-3f32);
        assert_approx_eq!(output[4], 0.0, 1e-5f32);

        output = Windowing::sine(&int_vector1, 5 );
        assert_approx_eq!(output[0], 0.0, 1e-5f32);
        assert_approx_eq!(output[1], 0.707, 1e-3f32);
        assert_approx_eq!(output[2], 1.0, 1e-3f32);
        assert_approx_eq!(output[3], 0.707, 1e-3f32);
        assert_approx_eq!(output[4], 0.0, 1e-5f32);
    }
    
    #[test]
    fn test_triangular(){

        let float_vector1 = RawData::FloatVec(vec![1.0;5]);
        let int_vector1 = RawData::IntVec(vec![1;5]);

        let mut output = Windowing::triangular(&float_vector1, 5 );
        assert_eq!(output, vec![0.0, 0.5, 1.0, 0.5, 0.0]);


        output = Windowing::triangular(&int_vector1, 5 );
        assert_eq!(output, vec![0.0, 0.5, 1.0, 0.5, 0.0]);

    }

}