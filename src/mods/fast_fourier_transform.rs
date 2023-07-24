use rustfft::{FftPlanner, num_complex::Complex};
use crate::RawData;

pub struct Fft{

    planner: FftPlanner<f32>,
}

impl Fft{

    //creates a new instance of fft with a planner to save memory and reduce setup time
    pub fn new() -> Self{
        Fft {
            planner: FftPlanner::new(),

        }
     }

     //converts data from RawData into Vec<f32> that has gone through a forward FFT - only returns real part
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

      //converts data from RawData into Vec<f32> that has gone through a forward FFT - only returns imaginary part
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

     //converts data from RawData into Vec<f32> that has gone through an inverse FFT- only returns real part
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

      //converts data from RawData into Vec<f32> that has gone through an inverse FFT - only returns imaginary part
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