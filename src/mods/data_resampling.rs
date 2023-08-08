use crate::RawData;

pub struct DataResampling {}

impl DataResampling{

    // A function that returns a new instance of the DataResampling struct
    pub fn new() -> Self{

        DataResampling {  }
    }

    // Function to perform resampling on input data by incrementing the index based on frequency ratios.
    // Input Parameters:
    // `self`: The function is associated with a struct, but it doesn't seem to use any struct-specific state.
    // `input`: A reference to the input data, which is an enum called `RawData` containing either a vector of f32 or i32 values.
    // `output_frequency`: The desired output frequency after resampling (in Hz).
    // `input_frequency`: The original input frequency of the data (in Hz).
    // Output:
    // A vector of f32 representing the resampled data based on the given output_frequency.
    pub fn resampling_by_average(&self, input: &RawData, output_frequency: f32, input_frequency: f32) -> Vec<f32>{
        match input{
            RawData::FloatVec(data_vector) =>{

                // Calculate the increment and output length
                let increment: f32 = (input_frequency/output_frequency).floor();
                let output_length: usize = (data_vector.len() as f32/increment).floor() as usize;
                
                //Initialize the output vector
                let mut output_vector: Vec<f32> = Vec::with_capacity(output_length);

                let mut index: f32 = 0.0;

                //Resampling by average
                for _ in 0..output_length {
                    let sum: f32 = data_vector[index as usize..(index + increment) as usize].iter().sum();
                    let avg: f32 = sum / increment as f32;
                    output_vector.push(avg);
            
                    index += increment;
                }
            
            
                output_vector

            }

            RawData::IntVec(data_vector) =>{

                //Convert integer vector to float vector
                let data_vector: Vec<f32> = data_vector.iter().map(|&x| x as f32).collect();
                
                //Calculate the incremenet and output length
                let increment = (input_frequency/output_frequency).floor();
                let output_length = (data_vector.len() as f32/increment).floor() as usize;
                
                //initialize the output vector
                let mut output_vector: Vec<f32> = Vec::with_capacity(output_length);

                let mut index: f32 = 0.0;
                
                //Resample by averaging
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

    // Function to perform resampling on input data by incrementing the index based on frequency ratios.
    // Input Parameters:
    // `self`: The function is associated with a struct, but it doesn't seem to use any struct-specific state.
    // `input`: A reference to the input data, which is an enum called `RawData` containing either a vector of f32 or i32 values.
    // `output_frequency`: The desired output frequency after resampling (in Hz).
    // `input_frequency`: The original input frequency of the data (in Hz).
    // Output:
    // A vector of f32 representing the resampled data based on the given output_frequency.
    pub fn resampling_by_increment(&self, input: &RawData, output_frequency: f32, input_frequency: f32) -> Vec<f32> {

        // Match the `input` data to determine if it contains a vector of f32 or i32 values.
         match input {
            // If the input data contains a vector of f32 values:
            RawData::FloatVec(data_vector) => {

                // Calculate the increment value by dividing input_frequency by output_frequency and rounding down.
                let increment = (input_frequency / output_frequency).floor() as usize;

                // Initialize variables for iterating and storing the resampled data.
                let mut index = 0;
                let mut output_vector: Vec<f32> = Vec::with_capacity(output_frequency as usize);

                // Loop through the data_vector, stepping by the calculated increment value.
                for index in (0..data_vector.len()).step_by(increment) {
                    // Push the data_vector value at the current index to the output_vector.
                    output_vector.push(data_vector[index]);
                }

                // Return the resampled output_vector.
                return output_vector;
            }

            // If the input data contains a vector of i32 values:
            RawData::IntVec(data_vector) => {

                // Convert the data_vector from i32 to f32 using map and collect.
                let data_vector: Vec<f32> = data_vector.iter().map(|&x| x as f32).collect();

                // Calculate the increment value by dividing input_frequency by output_frequency and rounding down.
                let increment = (input_frequency / output_frequency).floor() as usize;

                 // Initialize variables for iterating and storing the resampled data.
                let mut index = 0;
                let mut output_vector: Vec<f32> = Vec::with_capacity(output_frequency as usize);

                // Loop through the data_vector, stepping by the calculated increment value.
                for index in (0..data_vector.len()).step_by(increment) {
                    // Push the data_vector value at the current index to the output_vector.
                    output_vector.push(data_vector[index]);
                }

                // Return the resampled output_vector.
                return output_vector;
            }
    }
}

    //function that compares two vectors and ensures each element is approximately equal
    //eg, the expected vector will be in the expected parameter, the vector we recieved from the code will be in the actual parameter
    //and the epsilon parameter is the tolerance of how different the values can be
    pub fn assert_approx_equal_vec(expected: &[f32], actual: &[f32], epsilon: f32) {

        //confirms length of both vectors are the same
        assert_eq!(expected.len(), actual.len());

        //iterates over each value in the array and compares them by taking the difference of the values and ensuring absolute value is less than the epsilon
        for (e, a) in expected.iter().zip(actual.iter()) {
            assert!(
                (*e - *a).abs() < epsilon,
                "Element mismatch: expected {}, got {}",
                *e,
                *a
            );
        }
    }

}

#[cfg(test)]
mod tests {

    use ndarray::Data;

    use crate::RawData;
    use super::DataResampling;
    


    #[test]
    fn test_average(){

        //testing with positives
        let float_vector1: RawData= RawData::FloatVec(vec![0.91836795, -0.7550365, -0.82229968, 0.38470112, 0.32756556, -0.1500332,-0.47942242, 0.78735185, -0.3130074, -0.78962053, 0.68406937, -0.10941988,   0.85765562, -0.33443927, -0.88293069, -0.45943362, 0.7827355, -0.98156005,   0.1983309, 1.0490953, 0.78269412, 0.48753985, -0.4683345, -0.08328862,   0.47133147, 0.85514289, 0.02800922, 0.07371882, -0.28892876, 0.89355418,       0.20687296, -0.98254069, 0.73257495, 0.97115261, 0.88601853, 0.49660099,      -0.60699654, 0.99018712, 0.74853154, 1.03390956, 0.56113442, 0.93391565,     -0.34803287, -0.49468972, 0.54351342, 0.54288017, 0.56071839, -0.6816979,      0.44131403, 0.71416583, -0.20408451, -0.52847594, -0.86575061, 0.3189754,       0.03358667, -0.62728735, -0.19742717, -0.08853031, 0.82239147, 0.38570716,        -0.24534994, -0.55948792, 0.25647286, -0.60632081, 1.00364816, -0.40964857,      0.84938726, -0.54761025, 0.86841654, -0.9697848, 0.81779373, -0.74209949,      -0.01043969, 0.0437089, 0.91153038, 0.37510173, 0.6870717, -0.51927701, 0.22990968, -0.11206606, -0.70855284, -0.68672557, 0.34639136, 0.07711553, 0.74687533, -0.86588636, 0.60308048, 0.44784163, -0.48820096, -0.26549696,   0.6144216, 0.44270719, 0.28133413, -0.79150201, 0.05373624, -0.59698319, -0.44591376, 0.16755127, 0.27518268, 0.40648236]);
        let int_vector1 = RawData::IntVec(vec![1,-1,-1,0,0,0,0,1,0,-1,1,0,1,0,-1,0,1,-1,0,1,1,0,0,0,0,1,0,0,0,1,0,-1,1,1,1,0,-1,1,1,1,1,1,0,0,1,1,1,-1,0,1,0,-1,-1,0,0,-1,0,0,1,0,0,-1,0,-1,1,0,1,-1,1,-1,1,-1,0,0,1,0,1,-1,0,0,-1,-1,0,0,1,-1,1,0,0,0,1,0,0,-1,0,-1,0,0,0,0]);
        
        let float_vector2: RawData = RawData::FloatVec(vec![0.0,0.0,0.0]);
        let int_vector2: RawData = RawData::IntVec(vec![0,0,0]);

        let float_vector3: RawData= RawData::FloatVec(vec![57.42375095409937, 56.93672522014874, 62.608713592930876, 58.58999011614217, 53.781514473465975, 59.929765521680004, 62.83874316763905, 61.26564682933764, 60.298077069774514, 65.69037167656037, 64.4924856859658, 57.45776902834044, 67.2979270744406, 64.3065399558438, 69.10913177362237, 69.68135328937724, 66.27441648596799, 66.5712753857469, 68.7197206786976, 66.77069404089671, 69.47343229079385, 73.49687241835524, 74.39038713833476, 69.77134706156811, 71.11611036527033, 71.5514143700266, 70.92527568110486, 75.81690366006418, 71.48430850738842, 73.99189451125514, 77.46501916351879, 75.58651101737665, 78.48004995555802, 75.23963596784876, 80.30163806050568, 77.62973765421143, 80.16175629224077, 80.68837591441441, 79.6905673290241, 83.3649494980984, 83.41729893388869, 81.27997708388136, 82.92243382684352, 83.78103585804621, 83.22618159997842, 84.46526224102954, 84.07977328452601, 81.7653661597366, 83.24232423541409, 82.98821921592604, 84.52930477159329, 86.00457616832857, 84.85465642598282, 79.6146939926643, 83.91264690823981, 84.58107619775576, 86.86586732593057, 86.87782780765482, 86.49708394674948, 84.66183307481079, 83.32273574233504, 89.75815463910952, 81.73921292290746, 83.57667052875678, 82.29154311686115, 81.15090505024507, 81.0181809858243, 80.02406477550639, 79.95626138939767, 80.14968471195097, 77.93833929477205, 83.37140263453084, 80.63006474812504, 80.54886758093564, 74.61328386499513, 76.2067956336496, 75.93496294510105, 74.80986359211977, 79.09967283013746, 73.35452123905561, 76.43997860290371, 73.37084091593962, 72.27187673807346, 76.47512075935063, 73.51588873047531, 74.04609287428879, 73.5220350777692, 71.2657426722753, 69.51314201809633, 70.77160782734013, 68.74978122247276, 71.97834832913482, 68.52537097152933, 68.24617406288941, 69.73725303822009, 70.11807598587777, 69.35509863639831, 69.47650042355208, 66.64598319409271, 68.3345766746571, 66.2569569225079, 66.38695004387625, 58.302036498704005, 59.823489245315685, 63.90206250674178, 67.44946967703007, 64.55800828600442, 64.17137953752636, 64.24996006660133, 64.60722809289496, 62.715507684022455, 62.10266642672423, 62.987139113750466, 62.56118234068753, 60.74277369885567, 61.304485787724424, 63.39768612316597, 59.17504593185429, 60.92293919145394, 61.74329340412283, 57.66140451493842, 62.09807476069308, 61.33026367110421, 59.73879796926617, 58.5720628991695, 59.94647404967808, 62.000588448844795, 60.33024266169896, 60.98985504062868, 58.65602786741151, 59.89716687482642, 57.61316051537118, 57.166505274132426, 58.96556719641834, 60.37679949619507, 54.45584180986487, 58.7271982420652, 59.47366447708427, 56.07940328686498, 55.78066887807596, 54.54564631742577, 54.52126114421135, 56.12296587524522, 55.80474208024708, 55.450268698368774, 53.6860047610941, 56.265818477761954, 51.87808742443517, 54.7399559253531, 56.05143364518187, 52.41066187331751, 49.34598557487828, 50.79534970301276, 50.994619031875516, 54.5066732568543, 45.641877225921085, 45.44783149223058, 49.71813811826957, 49.06417606679746, 48.47113041190944, 45.066752680131536, 45.691797364781955, 46.04972969161036, 46.937126956810964, 48.579890629805696, 46.87831648338302, 43.2621571874902, 42.41539084532762, 47.732792000519055, 42.635078585330035, 45.25466510897015, 44.20457419707335, 39.80087116943825, 43.16784163357121, 42.72139421919673, 40.09690411968018, 39.072075680860785, 39.68721522523129, 39.69694747954145, 38.89958622251241, 36.20930810783846, 38.54348444090701, 39.49315655173681, 37.22306536922959, 38.45119902245094, 38.36169803033943, 36.27817076033729, 38.495487020334416, 36.08203266391632, 35.61234698124961, 37.31850254780822, 35.73508343583559, 34.24509993239897, 38.21302621383962, 35.21073369085666, 34.7557606412574, 38.30832823832005, 37.01263245066287, 38.27243482312153, 35.10563354463935, 36.54702691361821, 37.496545142093524, 40.21495219978659, 33.4472745629568, 36.19003918456128, 40.40279302834972, 39.782856300732334, 38.65349615550868, 39.553456446758894, 43.1239603503975, 38.65895285386617, 41.549607795356614, 41.87845934752343, 41.252783475633485, 41.294266760442, 42.2846785280957, 45.66429780888518, 48.37613972205177, 45.184550578912244, 45.83903707029868, 50.45023663869127, 47.807444589854164, 49.6770059662178, 47.06600589312027, 50.35579734602591, 45.08336916260292, 48.369671422972786, 50.56098595974319, 50.60044633370852, 54.91737419172761, 55.587993290196444, 51.32300551457022, 54.80025666186663, 51.74767705510797, 54.84209797727291, 56.02482448641735, 55.61823652859607, 58.55601631729386, 60.72812364016489, 58.75761743767916, 59.151794377943254, 59.22341098206529, 63.837931340247394, 61.3622177601643, 62.552959520063325, 62.90488582386602, 64.7298637786503, 64.29580055208083, 67.19411603468367]);        
        let int_vector3: RawData = RawData::IntVec(vec![57, 63, 59, 54, 60, 63, 61, 60, 66, 64, 57, 67, 64, 69, 70, 66, 67, 69, 67, 69, 73, 74, 70, 71, 72, 71, 76, 71, 74, 77, 76, 78, 75, 80, 78, 80, 81, 80, 83, 83, 81, 83, 84, 83, 84, 84, 82, 83, 83, 85, 86, 85, 80, 84, 85, 87, 87, 86, 85, 83, 90, 82, 84, 82, 81, 81, 80, 80, 80, 78, 83, 81, 81, 75, 76, 76, 75, 79, 73, 76, 73, 72, 76, 74, 74, 74, 71, 70, 71, 69, 72, 69, 68, 70, 70, 69, 69, 67, 68, 66, 66, 58, 60, 64, 67, 65, 64, 64, 65, 63, 62, 63, 63, 61, 61, 63, 59, 61, 62, 58, 62, 61, 60, 59, 60, 62, 60, 61, 59, 60, 58, 57, 59, 60, 54, 59, 59, 56, 56, 55, 55, 56, 56, 55, 54, 56, 52, 55, 56, 52, 49, 51, 51, 55, 46, 45, 50, 49, 48, 45, 46, 46, 47, 49, 47, 43, 42, 48, 43, 45, 44, 40, 43, 43, 40, 39, 40, 40, 39, 36, 39, 39, 37, 38, 38, 36, 38, 36, 36, 37, 36, 34, 38, 35, 35, 38, 37, 38, 35, 37, 37, 40, 33, 36, 40, 40, 39, 40, 43, 39, 42, 42, 41, 41, 42, 46, 48, 45, 46, 50, 48, 50, 47, 50, 45, 48, 51, 51, 55, 56, 51, 55, 52, 55, 56, 56, 59, 61, 59, 59, 59, 64, 61, 63, 63, 65, 64, 67]);
         


        let data: DataResampling = DataResampling::new();


        //testing with an empty array - ask what he wants 
        //let float_vector4 = RawData::FloatVec(vec![]);
        //let int_vector4 = RawData::IntVec(vec![]);

        
        //ensures its approximately equal by +-1 %
        DataResampling::assert_approx_equal_vec(&data.resampling_by_average(&float_vector1,33.0,100.0),&[-0.21965607666666664,0.18741116,-0.001692656666666681,-0.07165701333333332,-0.11990478,-0.21941939,0.6767067733333333,-0.021361089999999996,0.4514945266666666,0.22611474666666664,-0.014364260000000018,0.7845907099999999,0.3772407066666667,0.8429865433333332,-0.09973638999999997,0.14063355333333336,0.31713178333333336,-0.35841704999999996,-0.2637092833333333,0.37318944,-0.18278833333333333,-0.00410707333333334,0.3900645166666667,-0.29803018666666664,0.3149331966666667,0.18096547333333335,-0.19690307333333332,-0.08773956000000001,0.16135648333333333,-0.10195209666666667,0.44615430666666667,-0.44491632,-0.0010599366666666683], 0.0001);
        DataResampling::assert_approx_equal_vec(&data.resampling_by_average(&int_vector1,33.0,100.0),&[-0.3333333333333333,0.0,0.3333333333333333,0.0,0.0,0.0,0.6666666666666666,0.0,0.3333333333333333,0.3333333333333333,0.0,0.6666666666666666,0.3333333333333333,1.0,0.3333333333333333,0.3333333333333333,0.3333333333333333,-0.6666666666666666,-0.3333333333333333,0.3333333333333333,-0.3333333333333333,0.0,0.3333333333333333,-0.3333333333333333,0.3333333333333333,0.0,-0.3333333333333333,-0.3333333333333333,0.3333333333333333,0.0,0.3333333333333333,-0.6666666666666666,0.0], 0.0001);
       
        DataResampling::assert_approx_equal_vec(&data.resampling_by_average(&float_vector2,33.0,100.0),&[0.0], 0.0001);
        DataResampling::assert_approx_equal_vec(&data.resampling_by_average(&int_vector2,33.0,100.0),&[0.0], 0.0001);

        DataResampling::assert_approx_equal_vec(&data.resampling_by_average(&float_vector3,33.0,100.0),&[58.989729922392996,57.43375670376272,61.4674890222504,62.54687546362221,66.9045329346356,67.50901505369738,68.32128233679605,72.5528688727527,71.1976001388006,73.76436889290258,77.17719337881782,77.7236705608553,80.18023317855976,82.68740850528948,83.30988376162271,83.43680056176404,83.58661607431112,83.49130886232523,85.11986347730871,86.01224827640503,84.94003443478401,82.33970623195432,80.33283571690946,80.48647554708462,78.59740539801861,75.65054072362348,76.2980575573656,74.03927947112125,73.6946722275111,70.51683083923724,69.75116684104565,69.36716769566242,68.49252741801438,66.99282788034708,60.67586275025382,65.39295250018695,63.8575652811729,62.55032929372074,61.81498186991536,60.613759509143684,60.3632476489119,59.419111639371245,61.106895383724144,58.722118419203035,58.83629065558194,57.55223484300478,55.46857282745557,55.48298969990122,55.13403064574161,54.223158998323385,50.85066571706952,50.38105650488364,48.076715225765874,46.40989348560765,47.18891575940901,44.18528817206695,45.20751189827308,42.39109566669427,40.630124673245895,39.427916309095046,38.08198303349409,38.01198747400665,36.95189681486267,36.221977654964476,35.88961994569841,36.692240443413446,36.641698427126364,37.052923968278975,38.79189617121444,40.443637650888355,40.69567333224874,41.610576254723725,46.40832936994973,48.03223943294804,49.03293640178799,48.0046755151063,53.701937938544184,52.62364641051494,55.49505299742878,59.34725246504596,60.73771223341865,62.273354368031214, 65.4065933], 0.1);
        DataResampling::assert_approx_equal_vec(&data.resampling_by_average(&int_vector3,33.0,100.0),&[59.666666666666664,59.0,62.333333333333336,62.666666666666664,67.66666666666667,67.33333333333333,69.66666666666667,71.66666666666667,73.0,74.0,76.33333333333333,79.33333333333333,81.33333333333333,82.33333333333333,83.66666666666667,83.0,84.66666666666667,83.0,86.33333333333333,84.66666666666667,85.33333333333333,81.33333333333333,80.0,80.66666666666667,77.33333333333333,76.66666666666667,74.0,74.0,73.0,70.0,69.66666666666667,69.66666666666667,68.0,63.333333333333336,63.666666666666664,64.33333333333333,63.333333333333336,62.333333333333336,61.0,60.333333333333336,61.0,60.333333333333336,60.0,58.333333333333336,57.666666666666664,58.0,55.333333333333336,55.666666666666664,54.0,54.333333333333336,50.333333333333336,48.666666666666664,49.0,45.666666666666664,47.666666666666664,44.333333333333336,44.0,42.0,39.666666666666664,38.333333333333336,38.333333333333336,37.333333333333336,36.666666666666664,35.666666666666664,36.0,37.666666666666664,36.333333333333336,36.333333333333336,39.666666666666664,40.666666666666664,41.666666666666664,43.0,46.333333333333336,49.333333333333336,47.333333333333336,50.0,54.0,54.0,57.0,59.666666666666664,61.333333333333336,63.666666666666664], 0.1);
       


    }
}



