use crate::RawData;

pub struct PeakFinder{

}

impl PeakFinder{

    pub fn new() -> Self{
        PeakFinder { }
    }

    //returns the maximum value inside of the array
    //if array is empty, returns zero - check with sadra to see if he wants zero or the None in Option<f32>

    pub fn local_min(data: RawData) -> f32 {
        match data{
            RawData::FloatVec(data) =>{

                if data.is_empty() {
                    return 0.0; // Return None if the vector is empty
                }
            
                let mut min_value = data[0];
            
                for &value in data.iter() {
                    if value < min_value {
                        min_value = value;
                    }
                }
            
                min_value
            

                }

            RawData::IntVec(data) =>{
                
                let data: Vec<f32> = data.iter().map(|&x| x as f32).collect();
        
                if data.is_empty() {
                    return 0.0; // Return None if the vector is empty
                }
                
                let mut min_value = data[0];
                
                for &value in data.iter() {
                    if value < min_value {
                            min_value = value;
                    }
                }
                
                min_value
        
        
            }

        }


            }


        


        //returns the maximum value inside of the array
        //if array is empty, returns zero - check with sadra to see if he wants zero or the None in Option<f32>

        pub fn local_max(data: RawData) -> f32 {
match data{
            RawData::FloatVec(data) =>{

                if data.is_empty() {
                    return 0.0; // Return None if the vector is empty
                }
            
                let mut min_value = data[0];
            
                for &value in data.iter() {
                    if value < min_value {
                        min_value = value;
                    }
                }
            
                min_value

                }

            RawData::IntVec(data) =>{
                
                let data: Vec<f32> = data.iter().map(|&x| x as f32).collect();
        
                if data.is_empty() {
                    return 0.0; // Return None if the vector is empty
                }
                
                let mut min_value = data[0];
                
                for &value in data.iter() {
                    if value < min_value {
                            min_value = value;
                    }
                }
                
                min_value
        
        
            }

        }
    
    
    }


}


