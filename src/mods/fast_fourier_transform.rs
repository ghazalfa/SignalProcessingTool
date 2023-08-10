use rustfft::{FftPlanner, num_complex::Complex};
use crate::RawData;


/// A utility struct for performing Fast Fourier Transform (FFT) operations on raw data.
///
/// The `Fft` struct provides methods to compute forward and inverse FFTs on real or integer data.
/// It utilizes an FFT planner to optimize memory usage and reduce setup time, allowing users to
/// conveniently transform data between time and frequency domains.
///
/// # Example
///
/// ```rust
/// use complex::Complex;
/// use rustfft::{FftPlanner, num_traits::Float};
///
/// // Create a new FFT planner
/// let mut fft = Fft::new();
///
/// // Define input data as a Vec<f32>
/// let input_data: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
///
/// // Compute the forward FFT of the input data and extract the real and imaginary parts
/// let real_part = fft.run_forward_real(&RawData::FloatVec(input_data));
/// let imag_part = fft.run_forward_imag(&RawData::FloatVec(input_data));
///
/// // Perform inverse FFT on the real and imaginary parts to recover the original data
/// let recovered_data = fft.run_inverse_real(&RawData::FloatVec(real_part)) +
///                      fft.run_inverse_imag(&RawData::FloatVec(imag_part));
///
/// // Print the original and recovered data
/// println!("Original Data: {:?}", input_data);
/// println!("Recovered Data: {:?}", recovered_data);
/// ```
pub struct Fft{
    planner: FftPlanner<f32>,
}

impl Fft{

    /// Creates a new instance of `Fft` with a planner to save memory and reduce setup time.
    ///
    /// # Returns
    ///
    /// A new `Fft` instance.
    /// 
    pub fn new() -> Self{
        Fft {
            planner: FftPlanner::new(),

        }
     }

   /// Converts data from `RawData` into a `Vec<f32>` that has gone through a forward FFT.
    /// Only the real part of the result is returned.
    ///
    /// # Parameters
    ///
    /// - `input`: A reference to the input data of type `RawData`, which can be either `FloatVec` or `IntVec`.
    ///
    /// # Returns
    ///
    /// A `Vec<f32>` representing the real part of the result after a forward FFT.
    /// 
    pub fn run_forward_real(&mut self, input: &RawData ) -> Vec<f32>{

      match input{

         RawData::FloatVec(input) =>{

            let size: usize = input.len();

            //converts the Vec<f32> of data into Vec<Complex<f32>> so that it can be fed through the fft
            let mut complex_numbers: Vec<Complex<f32>> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();
            
            //Returns a Fft instance which computes forward FFTs of size len
            let fft = self.planner.plan_fft_forward(size);
    
            //processing the vector of complex numbers
            fft.process(&mut complex_numbers);
            
            //extracts real parts of the complex numbers and puts into a seperate array
            let mut real_part = Vec::with_capacity(complex_numbers.len());
            real_part.extend(complex_numbers.iter().map(|c| c.re));
    
            return real_part
         }

         RawData::IntVec(input) =>{
            let input: Vec<f32> = input.iter().map(|&x| x as f32).collect();

            let size: usize = input.len();

            //converts the Vec<f32> of data into Vec<Complex<f32>> so that it can be fed through the fft
            let mut complex_numbers: Vec<Complex<f32>> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();
            
            //Returns a Fft instance which computes forward FFTs of size len
            let fft = self.planner.plan_fft_forward(size);
    
            //processing the vector of complex numbers
            fft.process(&mut complex_numbers);
            
            //extracts real parts of the complex numbers and puts into a seperate array
            let mut real_part = Vec::with_capacity(complex_numbers.len());
            real_part.extend(complex_numbers.iter().map(|c| c.re));
    
            return real_part

         }


      }


     }
   

    /// Converts data from `RawData` into a `Vec<f32>` that has gone through a forward FFT.
    /// Only the imaginary part of the result is returned.
    ///
    /// # Parameters
    ///
    /// - `input`: A reference to the input data of type `RawData`, which can be either `FloatVec` or `IntVec`.
    ///
    /// # Returns
    ///
    /// A `Vec<f32>` representing the imaginary part of the result after a forward FFT.
    /// 
    pub fn run_forward_imag(&mut self, input: &RawData ) -> Vec<f32>{

         match input{
            RawData::FloatVec(input) =>{

               let size: usize = input.len();

               //converts the Vec<f32> of data into Vec<Complex<f32>> so that it can be fed through the fft
               let mut complex_numbers: Vec<Complex<f32>> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();
        
               //Returns a Fft instance which computes forward FFTs of size len
               let fft = self.planner.plan_fft_forward(size);

               //processing the vector of complex numbers
               fft.process(&mut complex_numbers);
        

               //extracts imaginary parts of the complex numbers and puts into a seperate array
               let mut imag_part = Vec::with_capacity(complex_numbers.len());
               imag_part.extend(complex_numbers.iter().map(|c| c.im));

               imag_part

            }
            RawData::IntVec(input) => {

               let input: Vec<f32> = input.iter().map(|&x| x as f32).collect();

               let size: usize = input.len();

               //converts the Vec<f32> of data into Vec<Complex<f32>> so that it can be fed through the fft
               let mut complex_numbers: Vec<Complex<f32>> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();
               
               //Returns a Fft instance which computes forward FFTs of size len
               let fft = self.planner.plan_fft_forward(size);
       
               //processing the vector of complex numbers
               fft.process(&mut complex_numbers);
               
       
               //extracts imaginary parts of the complex numbers and puts into a seperate array
               let mut imag_part = Vec::with_capacity(complex_numbers.len());
               imag_part.extend(complex_numbers.iter().map(|c| c.im));
       
               imag_part

            }


         }

  

     }

    /// Converts data from `RawData` into a `Vec<f32>` that has gone through an inverse FFT.
    /// Only the real part of the result is returned.
    ///
    /// # Parameters
    ///
    /// - `input`: A reference to the input data of type `RawData`, which can be either `FloatVec` or `IntVec`.
    ///
    /// # Returns
    ///
    /// A `Vec<f32>` representing the real part of the result after an inverse FFT.
    /// 
     pub fn run_inverse_real(&mut self, input: &RawData ) -> Vec<f32>{

      match input{
         RawData::FloatVec(input)=>{
 
            let size: usize = input.len();

            //converts the Vec<f32> of data into Vec<Complex<f32>> so that it can be fed through the fft
            let mut complex_numbers: Vec<Complex<f32>> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();
            
            //Returns a Fft instance which computes inverse FFTs of size len
            let fft = self.planner.plan_fft_inverse(size);
    
            //processing the vector of complex numbers
            fft.process(&mut complex_numbers);
            
            //extracts real parts of the complex numbers and puts into a seperate array        
            let mut real_part = Vec::with_capacity(complex_numbers.len());
            real_part.extend(complex_numbers.iter().map(|c| c.re));
    
            return real_part   
                   
         }
         RawData::IntVec(input)=>{
            let input: Vec<f32> = input.iter().map(|&x| x as f32).collect();
 
            let size: usize = input.len();

            //converts the Vec<f32> of data into Vec<Complex<f32>> so that it can be fed through the fft
            let mut complex_numbers: Vec<Complex<f32>> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();
            
            //Returns a Fft instance which computes inverse FFTs of size len
            let fft = self.planner.plan_fft_inverse(size);
    
            //processing the vector of complex numbers
            fft.process(&mut complex_numbers);
            
            //extracts real parts of the complex numbers and puts into a seperate array        
            let mut real_part = Vec::with_capacity(complex_numbers.len());
            real_part.extend(complex_numbers.iter().map(|c| c.re));
    
            return real_part       
        }
      }


     }

    /// Converts data from `RawData` into a `Vec<f32>` that has gone through an inverse FFT.
    /// Only the imaginary part of the result is returned.
    ///
    /// # Parameters
    ///
    /// - `input`: A reference to the input data of type `RawData`, which can be either `FloatVec` or `IntVec`.
    ///
    /// # Returns
    ///
    /// A `Vec<f32>` representing the imaginary part of the result after an inverse FFT.
     pub fn run_inverse_imag(&mut self, input: &RawData ) ->Vec<f32>{

         match input{
            RawData::FloatVec(input) =>{
   
               let size: usize = input.len();

               //converts the Vec<f32> of data into Vec<Complex<f32>> so that it can be fed through the fft
               let mut complex_numbers: Vec<Complex<f32>> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();
               
               //Returns a Fft instance which computes inverse FFTs of size len
               let fft = self.planner.plan_fft_inverse(size);
       
               //processing the vector of complex numbers
               fft.process(&mut complex_numbers);
       
               //extracts imaginary parts of the complex numbers and puts into a seperate array        
               let mut imag_part = Vec::with_capacity(complex_numbers.len());
               imag_part.extend(complex_numbers.iter().map(|c| c.im));
       
               return imag_part            
            }
            RawData::IntVec(input) => {
            let input: Vec<f32> = input.iter().map(|&x| x as f32).collect();
 
            let size: usize = input.len();

            //converts the Vec<f32> of data into Vec<Complex<f32>> so that it can be fed through the fft
            let mut complex_numbers: Vec<Complex<f32>> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();
            
            //Returns a Fft instance which computes inverse FFTs of size len
            let fft = self.planner.plan_fft_inverse(size);
    
            //processing the vector of complex numbers
            fft.process(&mut complex_numbers);
    
            //extracts imaginary parts of the complex numbers and puts into a seperate array        
            let mut imag_part = Vec::with_capacity(complex_numbers.len());
            imag_part.extend(complex_numbers.iter().map(|c| c.im));
    
            return imag_part           
            
            }
         }


     }    

}