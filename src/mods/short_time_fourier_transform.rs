

use crate::RawData;
use stft::{STFT, WindowType};

pub struct Stft{
    stft: STFT::<f64>

}

impl Stft{

    //initialized with a window type of Hanning
    pub fn new(window_size: f32, step_size: f32 ) -> Self {

        let window_type: WindowType = WindowType::Hanning;
        let mut stft = STFT::<f64>::new(window_type, window_size, step_size);

        Stft{
            stft
        }

    }

    pub fn run(&mut self, input: &RawData) -> Vec<Vec<f64>>{
            // we need a buffer to hold a computed column of the spectrogram
        let mut spectrogram_column: Vec<f64> =
        std::iter::repeat(0.).take(stft.output_size()).collect();
        //     a failed attempt to try it with cloning
        // let input = input.clone_vector();

        // let mut spectrogram: Vec<Vec<f64>> = Vec::new();

        // for some_samples in (&input[..]).chunks(3000) {
        //     // append the samples to the internal ringbuffer of the stft
        //     stft.append_samples(some_samples);
    
        //     // as long as there remain window_size samples in the internal
        //     // ringbuffer of the stft
        //     while stft.contains_enough_to_compute() {
        //         // compute one column of the stft by
        //         // taking the first window_size samples of the internal ringbuffer,
        //         // multiplying them with the window,
        //         // computing the fast fourier transform,
        //         // taking half of the symetric complex outputs,
        //         // computing the norm of the complex outputs and
        //         // taking the log10
        //         stft.compute_column(&mut spectrogram_column[..]);
    
        //         // here's where you would do something with the
        //         // spectrogram_column...
        //         spectrogram.push(spectrogram_column.clone());

    
        //         // drop step_size samples from the internal ringbuffer of the stft
        //         // making a step of size step_size
        //         stft.move_to_next_column();
        //     }
        // }

        // return spectrogram

        match input{
            RawData::FloatVec(input) => {
                let input: Vec<f64> = input.iter().map(|&x| x as f64).collect();


                let mut spectrogram: Vec<Vec<f64>> = Vec::new();

                for some_samples in (&input[..]).chunks(3000) {
                    // append the samples to the internal ringbuffer of the stft
                    stft.append_samples(some_samples);
            
                    // as long as there remain window_size samples in the internal
                    // ringbuffer of the stft
                    while stft.contains_enough_to_compute() {
                        // compute one column of the stft by
                        // taking the first window_size samples of the internal ringbuffer,
                        // multiplying them with the window,
                        // computing the fast fourier transform,
                        // taking half of the symetric complex outputs,
                        // computing the norm of the complex outputs and
                        // taking the log10
                        stft.compute_column(&mut spectrogram_column[..]);
            
                        // here's where you would do something with the
                        // spectrogram_column...
                        spectrogram.push(spectrogram_column.clone());

            
                        // drop step_size samples from the internal ringbuffer of the stft
                        // making a step of size step_size
                        stft.move_to_next_column();
                    }
                }

                return spectrogram
            }

            RawData::IntVec(input) =>{
                let input: Vec<f64> = input.iter().map(|&x| x as f64).collect();


                let mut spectrogram: Vec<Vec<f64>> = Vec::new();
                
                for some_samples in (&input[..]).chunks(3000) {
                    // append the samples to the internal ringbuffer of the stft
                    stft.append_samples(some_samples);
            
                    // as long as there remain window_size samples in the internal
                    // ringbuffer of the stft
                    while stft.contains_enough_to_compute() {
                        // compute one column of the stft by
                        // taking the first window_size samples of the internal ringbuffer,
                        // multiplying them with the window,
                        // computing the fast fourier transform,
                        // taking half of the symetric complex outputs,
                        // computing the norm of the complex outputs and
                        // taking the log10
                        stft.compute_column(&mut spectrogram_column[..]);
            
                        // here's where you would do something with the
                        // spectrogram_column...
                        spectrogram.push(spectrogram_column.clone());

            
                        // drop step_size samples from the internal ringbuffer of the stft
                        // making a step of size step_size
                        stft.move_to_next_column();
                    }
                }

                return spectrogram

            }
        }
    }
}