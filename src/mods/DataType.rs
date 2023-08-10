/// Enum that represents a vector of data in either `f32` or `i32` form.
/// 
/// #Example
/// 
/// ```rust
/// 
/// use mods::RawData;
/// let vec = vec![0.91836795, -0.7550365, -0.82229968, 0.38470112, 0.32756556, -0.1500332,-0.47942242];
/// let float_vector1: RawData= RawData::FloatVec(vec!);
/// 
///```rust
/// 
pub enum RawData {
    /// Represents a vector of `f32` (32-bit floating-point) values.
    FloatVec(Vec<f32>),
    /// Represents a vector of `i32` (32-bit signed integer) values.
    IntVec(Vec<i32>),
}

impl RawData{

    /// Returns a new `RawData` instance containing the provided `f32` vector.
    /// 
    /// # Example
    ///
    /// ```rust
    /// use mods::RawData;
    ///
    /// let float_vector = vec![1.0, 2.0, 3.0];
    /// let raw_data = RawData::new_float_vector(float_vector);
    /// ```
    /// 
    pub fn new_float_vector(vector: Vec<f32>) -> Self {
        RawData::FloatVec(vector)
    }


    /// Returns a new `RawData` instance containing the provided `i32` vector.
    /// 
    /// # Example
    ///
    /// ```rust
    /// use mods::RawData;
    ///
    /// let int_vector = vec![1, 2, 3];
    /// let raw_data = RawData::new_int_vector(int_vector);
    /// ```
    /// 
    pub fn new_int_vector(vector: Vec<i32>) -> Self {
        RawData::IntVec(vector)
    }

    ///Returns a clone of the vector inside the data struct as an 'f32' vector
    ///
    /// # Example
    ///
    /// ```rust
    /// use mods::RawData;
    ///
    /// let int_vector = vec![1, 2, 3];
    /// let raw_data = RawData::new_int_vector(int_vector);
    /// 
    /// let output: Vec<f32> = raw_data.return_vector_clone();
    /// ```
    /// 
    pub fn return_vector_clone(&self) -> Vec<f32>{
        match self {
            RawData::FloatVec(vector) => vector.clone(),
            RawData::IntVec(vector) => vector.iter().map(|&x| x as f32).collect(),
        }

    }
}
    

