use crate::RawData;

pub struct DataResampling {}

impl DataResampling{

    pub fn new() -> Self{

        DataResampling {  }
    }

    pub fn resampling_by_average(&self, input: &RawData, output_frequency: f32, input_frequency: f32) -> Vec<f32>{
        match input{
            RawData::FloatVec(data_vector) =>{

                let increment: f32 = (input_frequency/output_frequency).floor();
                let output_length: usize = (data_vector.len() as f32/increment).floor() as usize;
        
                let mut output_vector: Vec<f32> = Vec::with_capacity(output_length);

                let mut index: f32 = 0.0;
    
                for _ in 0..output_length {
                    let sum: f32 = data_vector[index as usize..(index + increment) as usize].iter().sum();
                    let avg: f32 = sum / increment as f32;
                    output_vector.push(avg);
            
                    index += increment;
                }
            
            
                output_vector

            }

            RawData::IntVec(data_vector) =>{
                let data_vector: Vec<f32> = data_vector.iter().map(|&x| x as f32).collect();
                let increment = (input_frequency/output_frequency).floor();
                let output_length = (data_vector.len() as f32/increment).floor() as usize;
        
                let mut output_vector: Vec<f32> = Vec::with_capacity(output_length);

                let mut index: f32 = 0.0;
    
                for _ in 0..output_length {
                    let sum: f32 = data_vector[index as usize..(index + increment) as usize].iter().sum();
                    let avg = sum / increment as f32;
                    output_vector.push(avg);
            
                    index += increment;
                }
            
            
                output_vector
            }
        }




    }

    pub fn resampling_by_increment(&self, input: &RawData, output_frequency: f32, input_frequency: f32) -> Vec<f32>{

        match input{
            RawData::FloatVec(data_vector) =>{
                let increment = (input_frequency/output_frequency).floor() as usize;


                let mut index = 0;
                let mut output_vector: Vec<f32> = Vec::with_capacity(output_frequency as usize);

                for index in (0..data_vector.len()).step_by(increment) {
                    output_vector.push(data_vector[index]);
                }

                 return output_vector;

            }

            RawData::IntVec(data_vector) =>{
                let data_vector: Vec<f32> = data_vector.iter().map(|&x| x as f32).collect();
                let increment = (input_frequency/output_frequency).floor() as usize;


                 let mut index = 0;
                 let mut output_vector: Vec<f32> = Vec::with_capacity(output_frequency as usize);

                 for index in (0..data_vector.len()).step_by(increment) {
                     output_vector.push(data_vector[index]);
                 }

                return output_vector;


            }
        }


    }

}





        // for _ in 0..output_length as i32{

        //     let mut avg: f32 = 0.0;

        //     for _ in 0..(increment as i32){
        //         avg +=data_vector[index];
        //         index+=1;

        //     }
        //     avg = avg/increment;
        //     output_vector.push(avg);
            
        // }

        // if remainder > 0.0 {

        //     for element in ((input_frequency-1.0) as i32-remainder as i32)..(input_frequency) as i32{
        //         output_vector.push(data_vector[element as usize]);
        //     }


        // }
