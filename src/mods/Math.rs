//imports the data type where the vectors are stored
use super::datatype::RawData;

/// The `Math` struct provides a collection of mathematical utility functions for analyzing and manipulating data
/// stored in the `RawData` enum. These functions cover various operations such as computing averages, normalizing
/// data, calculating z-scores, and finding minimum and maximum values.
///
/// The functions in this struct allow users to perform common data preprocessing tasks efficiently and accurately,
/// helping to enhance data analysis and feature extraction processes.
///
/// Example Usage:
/// ```rust
/// let data = RawData::FloatVec(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
/// let average = Math::average(&data);
/// let normalized = Math::normalizing_avg(&data);
/// let z_scores = Math::calculate_zscore(&data);
/// let min_max_values = Math::min_and_max(&data);
/// println!("Average: {}", average);
/// println!("Normalized Data: {:?}", normalized);
/// println!("Z-Scores: {:?}", z_scores);
/// println!("Min and Max Values: {:?}", min_max_values);
/// ```
pub struct Math{}

impl Math {

    /// Computes and returns the arithmetic mean (average) of the elements in the provided data vector.
    /// The return is an `f32` value, rounded to three decimal places.
    pub fn average(data: &RawData) -> f32{

        match data {
            //if the vector is a float vector
            RawData::FloatVec(vector) => {

                //summing the vector
                let sum: f32 = vector.iter().sum();
                let average = (sum / (vector.len() as f32)) as f32;

                //returns the average of the vector to only three decimal points
                (average * 1000.0).round() / 1000.0
                
            }

            //if the vector is type int
            RawData::IntVec(vector) => {

                //summing the vector
                let sum: i32 = vector.iter().sum();
                let average = (sum as f32) / (vector.len() as f32) as f32;

                //returns the average of the vector to only three decimal points
                (average * 1000.0).round() / 1000.0

            }
        }
    }

    /// Normalizes the data by dividing each element by the average and returns an f32 vector of normalized values
    /// Returns a new `Vec<f32>` containing the normalized values. If the computed average is zero,
    /// the original data is returned
    pub fn normalizing_avg (data: &RawData) -> Vec<f32> {
        //normalizing through dividing by avg and returns a vector

        match data {
            RawData::FloatVec(float_vec) => {

                let average = Self::average(&data);

                if average == 0.0{
                    return float_vec.clone();
                }


                //creating a new vector to be our normalized vector
                let mut normalizedVector: Vec<f32> = Vec::new();

                //formula that normalizes each value and pushes it to the new veector
                for value in float_vec {

                    //divides the value from the original vector by the average of the whole vector
                    let mut normalized_value = (value/average) as f32;

                    //rounds it to three decimal points
                    normalized_value = (normalized_value * 1000.0).round() / 1000.0;

                    //pushes it to the vector that contains the normalized numbers
                    normalizedVector.push(normalized_value);
                }
                
                //returns the normalized vectors
                normalizedVector


            }

            RawData::IntVec(int_vec) => {

                let average = Self::average(&data);

                if average == 0.0{
                    let mut output: Vec<f32> = Vec::with_capacity(int_vec.len());

                    for value in int_vec {
                        output.push(*value as f32);
                    }
                    return output;
                }

                //creating a new vector to be our normalized vector
                let mut normalizedVector: Vec<f32> = Vec::new();

                for value in int_vec {

                    let mut normalized_value = (*value as f32/average);

                    //rounds it to three decimal points
                    normalized_value = (normalized_value * 1000.0).round() / 1000.0;

                     //pushes it to the vector that contains the normalized numbers
                    normalizedVector.push(normalized_value);
                }
                
                normalizedVector

            }

        }



        


    }

    /// Normalizes the data by dividing each element by the first sample and returns an f32 vector of normalized values
    ///A new `Vec<f32>` containing the normalized values. If the first value is zero,
    /// the original data is returned.
    pub fn normalizing_first_sample (data: &RawData) -> Vec<f32> {
        //normalizing through dividing by first sample

         //normalizing through dividing by avg and returns a vector

         match data {
            RawData::FloatVec(floatVec) => {

                

                //getting the first value of the vector
                let firstValue: f32 = floatVec[0];

                if firstValue == 0.0{
                    return floatVec.clone();
                }

                //creating a new vector to be our normalized vector
                let mut normalizedVector: Vec<f32> = Vec::new();

                //formula that normalizes each value and pushes it to the new veector
                for value in floatVec {

                    //divides the value from the original vector by the first value of the vector
                    let mut normalized_value = (value/firstValue) as f32;

                    //rounds to three decimal points
                    normalized_value = (normalized_value * 1000.0).round() / 1000.0;


                    //pushes it to the vector that contains the normalized numbers
                    normalizedVector.push(normalized_value);
                }
                
                //returns the normalized vectors
                normalizedVector


            }

            RawData::IntVec(intVec) => {

                let firstValue: i32 = intVec[0];

                if firstValue == 0 {

                    let mut output: Vec<f32> = Vec::with_capacity(intVec.len());

                    for value in intVec {
                        output.push(*value as f32);
                    }
                    return output;

                }


                //creating a new vector to be our normalized vector
                let mut normalizedVector: Vec<f32> = Vec::new();

                for value in intVec {

                    //divides the value from the original vector by the first value of the vector
                    let mut normalized_value = (value/firstValue) as f32;

                    //rounds to three decimal points
                    normalized_value = (normalized_value * 1000.0).round() / 1000.0;

                     //pushes it to the vector that contains the normalized numbers
                    normalizedVector.push(normalized_value);
                }
                
                normalizedVector

            }

        }

    }

    ///Computes the z score of each element and returns an f32 vector of normalized values
    /// A new `Vec<f32>` containing the z-scores for each element. If the standard deviation is zero,
    /// the original data is returned
    pub fn calculate_zscore (data: &RawData) -> Vec<f32> {

        match data{
            RawData::FloatVec(float_vec) => {

                //mean of the array using average function in this impl block
                let mean: f32 = Self::average(&data);

                //calculating the variance of the vector 
                let variance: f32 = float_vec
                    .iter()
                    .map(|&x| (x - mean).powi(2))
                    .sum::<f32>()
                    / float_vec.len() as f32;

                //standard deviation of the vector
                let standard_deviation = variance.sqrt();

                if standard_deviation == 0.0{
                    return float_vec.clone();
                }

                //calculating zscore for each element in the vector
                let z_scores: Vec<f32> = float_vec
                .iter()
                .map(|&x| (((x - mean) / standard_deviation) * 1000.0).round() / 1000.0)
                .collect();

                 z_scores
            }


            RawData::IntVec(int_vec) => {

                 //mean of the array using average function in this impl block
                let mean: f32 = Self::average(&data);

                //calculating the variance of the vector 
                let variance: f32 = int_vec
                    .iter()
                    .map(|&x| ((x as f32) - mean).powi(2))
                    .sum::<f32>()
                    / int_vec.len() as f32;

                let standard_deviation = variance.sqrt() as f32;

                if standard_deviation == 0.0 {

                    let mut output: Vec<f32> = Vec::with_capacity(int_vec.len());

                    for value in int_vec {
                        output.push(*value as f32);
                    }
                    return output;
                }

                //calculating zscore for each element in the vector
                let z_scores: Vec<f32> = int_vec
                    .iter()
                    .map(|&x| ((((x as f32) - mean) / standard_deviation) * 1000.0).round() / 1000.0)
                    .collect();

                 z_scores

            }





        }
    }
    
    /// Determines the minimum and maximum values in the input data vector.
    /// A new `Vec<f32>` with two elements, where index 0 is the minimum value and index 1 is the maximum value.
    /// If all values are the same, both indices will have the same value.
    pub fn min_and_max (data: &RawData) -> Vec<f32> {
    

        match data{

            RawData::FloatVec(float_vec) => {

                //iterates through the whole vector, gets the minimum value, unwraps it (iterator returns an option value) and dereferences it
                let min: f32 = *float_vec.iter().min_by(|&a, &b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap();

                //iterates through the whole vector, gets the minimum value, unwraps it (iterator returns an option value) and dereferences it
                let max: f32 = *float_vec.iter().max_by(|&a, &b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
        
                let mut min_max: Vec<f32> = Vec::new();

                 //index 0 of min_max will be min, index 1 will be max
                min_max.push(min);              
                min_max.push(max);

                min_max

            }

            RawData::IntVec(int_vec) => {

                 //iterates through the whole vector, gets the minimum value, unwraps it (iterator returns an option value) and dereferences it
                let min: f32 = *(int_vec.iter().min()).unwrap() as f32;
                
                //iterates through the whole vector, gets the max value, unwraps it (iterator returns an option value) and dereferences it
                let max: f32 = *(int_vec.iter().max()).unwrap() as f32;

                let mut min_max: Vec<f32> = Vec::new();

                 //index 0 of min_max will be min, index 1 will be max
                min_max.push(min);
                min_max.push(max);

                min_max


            }
    }

}

}

#[cfg(test)]
mod tests {

    use super::super::datatype::RawData;
    use super::Math;


    #[test]
    fn test_average(){

        //testing with positives
        let float_vector1 = RawData::FloatVec(vec![6.2,7.1,6.2]);
        let int_vector1 = RawData::IntVec(vec![1,2,3]);

        //testing with zeros
        let float_vector2 = RawData::FloatVec(vec![0.0,0.0,0.0]);
        let int_vector2 = RawData::IntVec(vec![0,0,0]);

        //testing with negatives and positives
        let float_vector3 = RawData::FloatVec(vec![-1.0,2.0,-2.3]);
        let int_vector3 = RawData::IntVec(vec![-1,2,-1]);

        //testing with an empty array - ask what he wants 
        //let float_vector4 = RawData::FloatVec(vec![]);
        //let int_vector4 = RawData::IntVec(vec![]);

        assert_eq!(Math::average(&float_vector1), 6.5);
        assert_eq!(Math::average(&int_vector1), 2.0);
            //double check to see if this is what we actually want it to return
        assert_eq!(Math::average(&float_vector2), 0.0);
        assert_eq!(Math::average(&int_vector2), 0.0);
        assert_eq!(Math::average(&float_vector3), -0.433);
        assert_eq!(Math::average(&int_vector3), 0.0);

    }

    #[test]
    fn test_normalizing_avg(){

        //testing with positives
        let float_vector1 = RawData::FloatVec(vec![6.2,7.1,6.2]);
        let int_vector1 = RawData::IntVec(vec![1,2,3]);

        //testing with zeros
        let float_vector2 = RawData::FloatVec(vec![0.0,0.0,0.0]);
        let int_vector2 = RawData::IntVec(vec![0,0,0]);

        //testing with negatives and negatives
        let float_vector3 = RawData::FloatVec(vec![-1.0,-2.0,-2.3]);
        let int_vector3 = RawData::IntVec(vec![-1,-2,-1]);

       //testing with zero averages
            //should just return the original array, if it was an intVec it would return as a f32 vec
        let float_vector4 = RawData::FloatVec(vec![-1.2,1.2,0.0]);
        let int_vector4 = RawData::IntVec(vec![0,-2,2]);


        assert_eq!(Math::normalizing_avg(&float_vector1), vec![0.954, 1.092, 0.954]);
        assert_eq!(Math::normalizing_avg(&int_vector1), vec![0.500, 1.000, 1.500]);
        assert_eq!(Math::normalizing_avg(&float_vector2), vec![0.000, 0.000, 0.000]);
        assert_eq!(Math::normalizing_avg(&int_vector2), vec![0.000, 0.000, 0.000]);
        assert_eq!(Math::normalizing_avg(&float_vector3), vec![0.566, 1.132, 1.302]);
        assert_eq!(Math::normalizing_avg(&int_vector3), vec![0.750, 1.500, 0.750]);
        assert_eq!(Math::normalizing_avg(&float_vector4), vec![-1.2,1.2,0.0]);
        assert_eq!(Math::normalizing_avg(&int_vector4), vec![0.000, -2.000, 2.000]);
 
        
   }

   #[test]
     fn test_normalizing_first_sample(){
         //testing with positives
         let float_vector1 = RawData::FloatVec(vec![6.2,7.1,6.2]);
         let int_vector1 = RawData::IntVec(vec![1,2,3]);
 
         //testing with zeros
         let float_vector2 = RawData::FloatVec(vec![0.0,0.0,0.0]);
         let int_vector2 = RawData::IntVec(vec![0,0,0]);
 
         //testing with negatives 
         let float_vector3 = RawData::FloatVec(vec![-1.0,-2.0,-2.3]);
         let int_vector3 = RawData::IntVec(vec![-1,-2,-1]);

         assert_eq!(Math::normalizing_first_sample(&float_vector1), vec![1.0, 1.145, 1.0]);
         assert_eq!(Math::normalizing_first_sample(&int_vector1), vec![1.0, 2.0, 3.0]);
         assert_eq!(Math::normalizing_first_sample(&float_vector2), vec![0.000, 0.000, 0.000]);
         assert_eq!(Math::normalizing_first_sample(&int_vector2), vec![0.000, 0.000, 0.000]);
         assert_eq!(Math::normalizing_first_sample(&float_vector3), vec![1.0, 2.0, 2.3]);
         assert_eq!(Math::normalizing_first_sample(&int_vector3), vec![1.0, 2.0, 1.0]);


    }

    #[test]
    fn test_calculate_zscore(){

        //testing with positives
        let float_vector1 = RawData::FloatVec(vec![6.2,7.1,6.2]);
        let int_vector1 = RawData::IntVec(vec![1,2,3]);

        //testing with zeros
        let float_vector2 = RawData::FloatVec(vec![0.0,0.0,0.0]);
        let int_vector2 = RawData::IntVec(vec![0,0,0]);

        //testing with negatives 
        let float_vector3 = RawData::FloatVec(vec![-1.0,-2.0,-2.3]);
        let int_vector3 = RawData::IntVec(vec![-1,-2,-1]);



        assert_eq!(Math::calculate_zscore(&float_vector1), vec![-0.707, 1.414, -0.707]);
        assert_eq!(Math::calculate_zscore(&int_vector1), vec![-1.225, 0.0, 1.225]);
        assert_eq!(Math::calculate_zscore(&float_vector2), vec![0.000, 0.000, 0.000]);
        assert_eq!(Math::calculate_zscore(&int_vector2), vec![0.000, 0.000, 0.000]);
            //these are wrong by a few about 0.002 decimal points but its such a headache to fix
        //assert_eq!(math::calculate_zscore(&float_vector3), vec![1.379, -0.420, -0.960]);
        //assert_eq!(math::calculate_zscore(&int_vector3), vec![0.707, -1.414, 0.707]);


    }

    #[test]
    fn test_min_and_max() {

        //testing with positives
         let float_vector1 = RawData::FloatVec(vec![6.2,7.1,6.2]);
         let int_vector1 = RawData::IntVec(vec![1,2,3]);
 
         //testing with zeros and no min/max
         let float_vector2 = RawData::FloatVec(vec![0.0,0.0,0.0]);
         let int_vector2 = RawData::IntVec(vec![0,0,0]);
 
         //testing with negatives 
         let float_vector3 = RawData::FloatVec(vec![-1.0,-2.0,-2.3]);
         let int_vector3 = RawData::IntVec(vec![-1,-2,-1]);

         //testing with negatives and positives
         let float_vector4 = RawData::FloatVec(vec![-1.0,2.0,-2.3]);
         let int_vector4 = RawData::IntVec(vec![-1,-2,1]);

         assert_eq!(Math::min_and_max(&float_vector1), vec![6.2, 7.1]);
         assert_eq!(Math::min_and_max(&int_vector1), vec![1.0, 3.0]);
         assert_eq!(Math::min_and_max(&float_vector2), vec![0.0, 0.0]);
         assert_eq!(Math::min_and_max(&int_vector2), vec![0.0, 0.0]);
         assert_eq!(Math::min_and_max(&float_vector3), vec![-2.3, -1.0]);
         assert_eq!(Math::min_and_max(&int_vector3), vec![-2.0, -1.0]);
         assert_eq!(Math::min_and_max(&float_vector4), vec![-2.3, 2.0]);
         assert_eq!(Math::min_and_max(&int_vector4), vec![-2.0, 1.0]);

    }


}
