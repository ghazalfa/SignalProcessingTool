use rustfft::{FftPlanner, num_complex::Complex};
use crate::RawData;

pub struct fft{

    planner: FftPlanner<f32>,
}

impl fft{

    //creates a new instance of fft with a planner to save memory and reduce setup time
    pub fn new() -> Self{
        fft {
            planner: FftPlanner::new(),

        }
     }


     //converts data from RawData into Vec<Complex<f32>> that has gone through a forward FFT
     pub fn run_forward(&mut self, input: &RawData ) -> Vec<Complex<f32>>{

        let input: Vec<f32> = input.clone_vector();

        let size: usize = input.len();

        //converts the Vec<f32> of data into Vec<Complex<f32>> so that it can be fed through the fft
        let mut complex_numbers: Vec<Complex<f32>> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();
        
        //Returns a Fft instance which computes forward FFTs of size len
        let fft = self.planner.plan_fft_forward(size);

        //processing the vector of complex numbers
        fft.process(&mut complex_numbers);
        
        //returning the mutated vector of complex numbers
        complex_numbers

     }

     //converts data from RawData into Vec<Complex<f32>> that has gone through an inverse FFT
     pub fn run_inverse(&mut self, input: &RawData ) -> Vec<Complex<f32>>{

        let input: Vec<f32> = input.clone_vector();

        let size: usize = input.len();

        //converts the Vec<f32> of data into Vec<Complex<f32>> so that it can be fed through the fft
        let mut complex_numbers: Vec<Complex<f32>> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();
        
        //Returns a Fft instance which computes inverse FFTs of size len
        let fft = self.planner.plan_fft_inverse(size);

        //processing the vector of complex numbers
        fft.process(&mut complex_numbers);
        
        //returning the mutated vector of complex numbers
        complex_numbers

     }
}