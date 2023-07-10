
use super::datatype::RawData;
use dsp::window;


pub struct Windowing{}

impl Windowing{

    /// The `windowing` function takes a `frame` of data type RawData and an `increment` value. It splits the frame
    /// into sub-vectors of length equal to the increment and returns a vector of these sub-vectors.
    /// The sub-vectors are represented as `Vec<Vec<f32>>`.
    pub fn windowing(frame: &RawData, increment: i32) -> Vec<Vec<f32>>{

        match frame{
            //if the vector is a float vector
            RawData::FloatVec(vector) => {

                let sub_vectors: Vec<Vec<f32>> = vector.chunks((increment as usize))
                                            .map(|chunk| chunk.to_vec())
                                            .collect();
                
                return sub_vectors;

            }

            //if the vector is type int
            RawData::IntVec(vector) => {

                let sub_vectors: Vec<Vec<f32>> = vector.chunks((increment as usize))
                                                .map(|chunk| chunk.iter().map(|&x| x as f32).collect())
                                                .collect();

                return sub_vectors;

            }

        }


    }

    /// The `rectangular` function takes a `frame` of data type RawData and a `size` value. It applies a rectangular
    /// window of the specified size to the frame and returns a vector representing the windowed frame.
    /// The windowed frame is represented as `Vec<f32>`.
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

    /// The `blackman` function takes a `frame` of data type RawData and a `size` value. It applies a Blackman
    /// window of the specified size to the frame and returns a vector representing the windowed frame.
    // The windowed frame is represented as `Vec<f32>`.
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
    
    /// The `hamming` function takes a `frame` of data type RawData and a `size` value. It applies a Hamming
    /// window of the specified size to the frame and returns a vector representing the windowed frame.
    /// The windowed frame is represented as `Vec<f32>`
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

    /// The `hann` function takes a `frame` of data type RawData and a `size` value. It applies a Hann
    /// window of the specified size to the frame and returns a vector representing the windowed frame.
    /// The windowed frame is represented as `Vec<f32>`.
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

    /// The `welch` function takes a `frame` of data type RawData and a `size` value. It applies a Welch
    /// window of the specified size to the frame and returns a vector representing the windowed frame.
    /// The windowed frame is represented as `Vec<f32>`.
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

    /// The `sine` function takes a `frame` of data type RawData and a `size` value. It applies a sine
    /// window of the specified size to the frame and returns a vector representing the windowed frame.
    /// The windowed frame is represented as `Vec<f32>`.
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

    /// The `triangular` function takes a `frame` of data type RawData and a `size` value. It applies a triangular
    /// window of the specified size to the frame and returns a vector representing the windowed frame.
    /// The windowed frame is represented as `Vec<f32>`.
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

// #[cfg(test)]
// mod tests {

//     use super::super::DataType::RawData;
//     use super::Windowing;
//     use dsp::window;
//     use assert_approx_eq::assert_approx_eq;


//     #[test]
//     fn test_windowing(){

//         let float_vector1 = RawData::FloatVec(vec![6.2,7.1,6.2]);
//         let int_vector1 = RawData::IntVec(vec![1,2,3]);

//         assert_eq!(Windowing::windowing(&float_vector1,2), vec![vec![6.2,7.1],vec![6.2]]);
//         assert_eq!(Windowing::windowing(&int_vector1,2), vec![vec![1.0,2.0],vec![3.0]]);


//     }

//     #[test]
//     fn test_rectangular(){

//         let float_vector1 = RawData::FloatVec(vec![1.0;3]);
//         let int_vector1 = RawData::IntVec(vec![1;3]);


//         assert_eq!(Windowing::rectangular(&float_vector1, 3), vec![1.0, 1.0, 1.0]);
//         assert_eq!(Windowing::rectangular(&int_vector1, 3), vec![1.0, 1.0, 1.0]);

//     }

//     #[test]
//     fn test_blackman(){

//         let float_vector1 = RawData::FloatVec(vec![1.0;5]);
//         let int_vector1 = RawData::IntVec(vec![1;5]);

//         let mut output = Windowing::blackman(&float_vector1, 5 );

//         assert_approx_eq!(output[0], 0.00687, 1e-5f32);
//         assert_approx_eq!(output[1], 0.34974, 1e-5f32);
//         assert_approx_eq!(output[2], 1.0, 1e-5f32);
//         assert_approx_eq!(output[3], 0.34974, 1e-5f32);
//         assert_approx_eq!(output[4], 0.00687, 1e-5f32);

//         output = Windowing::blackman(&int_vector1, 5 );

//         assert_approx_eq!(output[0], 0.00687, 1e-5f32);
//         assert_approx_eq!(output[1], 0.34974, 1e-5f32);
//         assert_approx_eq!(output[2], 1.0, 1e-5f32);
//         assert_approx_eq!(output[3], 0.34974, 1e-5f32);
//         assert_approx_eq!(output[4], 0.00687, 1e-5f32);


//     }
    
//     #[test]   
//     fn test_hamming(){

//         let float_vector1 = RawData::FloatVec(vec![1.0;5]);
//         let int_vector1 = RawData::IntVec(vec![1;5]);

//         let mut output = Windowing::hamming(&float_vector1, 5 );

//         assert_approx_eq!(output[0], 0.0869, 1e-3f32);
//         assert_approx_eq!(output[1], 0.54347825, 1e-3f32);
//         assert_approx_eq!(output[2], 1.0, 1e-3f32);
//         assert_approx_eq!(output[3], 0.54347825, 1e-3f32);
//         assert_approx_eq!(output[4], 0.0869, 1e-3f32);

//         output = Windowing::hamming(&int_vector1, 5 );

//         assert_approx_eq!(output[0], 0.0869, 1e-3f32);
//         assert_approx_eq!(output[1], 0.54347825, 1e-3f32);
//         assert_approx_eq!(output[2], 1.0, 1e-3f32);
//         assert_approx_eq!(output[3], 0.54347825, 1e-3f32);
//         assert_approx_eq!(output[4], 0.0869, 1e-3f32);


//     }

//     #[test]   
//     fn test_hann(){

//         let float_vector1 = RawData::FloatVec(vec![1.0;5]);
//         let int_vector1 = RawData::IntVec(vec![1;5]);

//         let mut output = Windowing::hann(&float_vector1, 5 );
        
//         assert_approx_eq!(output[0], 0.0, 1e-5f32);
//         assert_approx_eq!(output[1], 0.5, 1e-3f32);
//         assert_approx_eq!(output[2], 1.0, 1e-3f32);
//         assert_approx_eq!(output[3], 0.5, 1e-3f32);
//         assert_approx_eq!(output[4], 0.0, 1e-5f32);

//         output = Windowing::hann(&int_vector1, 5 );

//         assert_approx_eq!(output[0], 0.0, 1e-5f32);
//         assert_approx_eq!(output[1], 0.5, 1e-3f32);
//         assert_approx_eq!(output[2], 1.0, 1e-3f32);
//         assert_approx_eq!(output[3], 0.5, 1e-3f32);
//         assert_approx_eq!(output[4], 0.0, 1e-5f32);


//     }

//     #[test]   
//     fn test_welch(){

//         let float_vector1 = RawData::FloatVec(vec![1.0;5]);
//         let int_vector1 = RawData::IntVec(vec![1;5]);

//         let mut output = Windowing::welch(&float_vector1, 5 );
//         assert_eq!(output, vec![0.0, 0.75, 1.0, 0.75, 0.0]);

//         output = Windowing::welch(&int_vector1, 5 );
//         assert_eq!(output, vec![0.0, 0.75, 1.0, 0.75, 0.0]);

//     }
   
//     #[test]   
//     fn test_sine(){

//         let float_vector1 = RawData::FloatVec(vec![1.0;5]);
//         let int_vector1 = RawData::IntVec(vec![1;5]);

//         let mut output = Windowing::sine(&float_vector1, 5 );
//         assert_approx_eq!(output[0], 0.0, 1e-5f32);
//         assert_approx_eq!(output[1], 0.707, 1e-3f32);
//         assert_approx_eq!(output[2], 1.0, 1e-3f32);
//         assert_approx_eq!(output[3], 0.707, 1e-3f32);
//         assert_approx_eq!(output[4], 0.0, 1e-5f32);

//         output = Windowing::sine(&int_vector1, 5 );
//         assert_approx_eq!(output[0], 0.0, 1e-5f32);
//         assert_approx_eq!(output[1], 0.707, 1e-3f32);
//         assert_approx_eq!(output[2], 1.0, 1e-3f32);
//         assert_approx_eq!(output[3], 0.707, 1e-3f32);
//         assert_approx_eq!(output[4], 0.0, 1e-5f32);
//     }
    
//     #[test]
//     fn test_triangular(){

//         let float_vector1 = RawData::FloatVec(vec![1.0;5]);
//         let int_vector1 = RawData::IntVec(vec![1;5]);

//         let mut output = Windowing::triangular(&float_vector1, 5 );
//         assert_eq!(output, vec![0.0, 0.5, 1.0, 0.5, 0.0]);


//         output = Windowing::triangular(&int_vector1, 5 );
//         assert_eq!(output, vec![0.0, 0.5, 1.0, 0.5, 0.0]);

//     }

// }