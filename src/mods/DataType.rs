
//enum that keeps the vector of data in either f32 or i32 form
pub enum RawData{
    FloatVec(Vec<f32>),
    IntVec(Vec<i32>),
}

impl RawData{

    pub fn return_vector(&self) -> Vec<f32>{
        match self {
            RawData::FloatVec(vector) => vector.clone(),
            RawData::IntVec(vector) => vector.iter().map(|&x| x as f32).collect(),
        }

    }

    

    pub fn clone_vector(&self) -> Vec<f32>{

        match self {
            RawData::FloatVec(vector) => vector.clone(),
            RawData::IntVec(vector) => vector.iter().map(|&x| x as f32).collect(),
        }

    }
}
