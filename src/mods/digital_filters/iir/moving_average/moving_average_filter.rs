use crate::mods::datatype::RawData;


/// A struct representing a moving average filter
///
///This struct is used to create and process a moving average filter on data provided
///in the form of `RawData` containing either `f32` or `i32` values
pub struct MovingAverageFilter{}

impl MovingAverageFilter {

    /// Creates a new instance of the moving average filter
    /// 
    /// # Returns
    /// 
    /// An instance of the 'moving_average_filter' struct
    /// 
    /// # Examples
    /// ```
    /// use crate::mods::datatype::RawData;
    /// use crate::mods::digital_filters::iir::moving_average::MovingAverageFilter;
    /// 
    /// let mut filter = moving_average_filter::new();
    /// 
    /// ```

    pub fn new() -> Self{
        MovingAverageFilter {}
     }

    
    /// Processes the input data using a moving average filter and returns the filtered output.
    ///
    /// This filter computes the moving average of each element in the input data by summing the
    /// element itself and its adjacent neighbors (i-1 and i+1), and then dividing by three.
    ///
    /// For the first and last elements, since there are no neighbors on one side, the missing neighbor
    /// is assumed to be zero.
    ///
    /// # Arguments
    ///
    /// * `input` - A reference to the input data of type `RawData`, which can be either `FloatVec` or `IntVec`.
    ///
    /// # Returns
    ///
    /// A `Vec<f32>` containing the filtered output data.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::mods::datatype::RawData;
    /// use crate::mods::digital_filters::iir::moving_average::MovingAverageFilter;
    /// 
    /// let mut filter = MovingAverageFilter::new();
    /// let input_data = RawData::FloatVec(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    /// let output = filter.process(&input_data);
    /// assert_eq!(output, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    /// ```
    /// 
    pub fn process(&mut self, input: &RawData ) -> Vec<f32>{

        match input {
            RawData::FloatVec(input)=>{
                let size = input.len();
                let size = size;
                let mut output_vec1: Vec<f32> = Vec::with_capacity(input.len());
        
                //calculating for the first element
                let mut element = (input[0]+input[1])/3.0;
                //output_vec1.push(element);
        
                //everything inbetween first and last element exclusive
                for i in 1..size-1{
        
                     element = (input[i-1]+input[i]+input[i+1])/3.0;
                    output_vec1.push(element);
        
                }
        
                //calculating the last element
                element = (input[size-2]+input[size-1])/3.0;
        
                //output_vec1.push(element);
        
                //returning the vector
                output_vec1

            }

            RawData::IntVec(input) =>{

                let input: Vec<f32> = input.iter().map(|&x| x as f32).collect();

                let size = input.len();
                let size = size;
                let mut output_vec1: Vec<f32> = Vec::with_capacity(input.len());
        
                //calculating for the first element
                let mut element = (input[0]+input[1])/3.0;
                //output_vec1.push(element);
        
                //everything inbetween first and last element exclusive
                for i in 1..size-1{
        
                     element = (input[i-1]+input[i]+input[i+1])/3.0;
                    output_vec1.push(element);
        
                }
        
                //calculating the last element
                element = (input[size-2]+input[size-1])/3.0;
        
                //output_vec1.push(element);
        
                //returning the vector
                output_vec1

            }
        }




    }

}

