
//struct to create a moving average filter
pub struct moving_average_filter{

}

impl moving_average_filter {

    pub fn new() -> Self{
        moving_average_filter {}
     }
    
     //filter will sum elements i-1, i, and i+1, and then divide by three
     //in the case of the first and last elements, i-1 will be assumed to be zero and i+1 will be assumed to be zera
    pub fn process(&mut self, input: &Vec<f32> ) -> Vec<f32>{
        let size = input.len();
        let size = size;
        let mut output_vec1: Vec<f32> = Vec::new();

        //calculating for the first element
        let mut element = (input[0]+input[1])/3.0;
        output_vec1.push(element);

        //everything inbetween first and last element exclusive
        for i in 1..size-1{

             element = (input[i-1]+input[i]+input[i+1])/3.0;
            output_vec1.push(element);

        }

        //calculating the last element
        element = (input[size-2]+input[size-1])/3.0;

        output_vec1.push(element);

        //returning the vector
        output_vec1

    }

}

