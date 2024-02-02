# SpectraEngine

## Table of Contents

Signal processing is a critical component of many applications in areas such as telecommunications,
audio processing, image processing, and biomedical engineering. The development of a high-performance signal-
processing tool in Rust has the potential to offer developers and researchers an efficient, reliable, and flexible tool for a
wide range of signal-processing tasks.

In this project, I developed a signal processing tool written in Rust to integrate 
with the Shadgan Labs cross-platform data acquisition near-infrared spectroscopy software (Spectrum).

The objective of the tool is to leverage the power and safety of the Rust programming language to create a fast and reliable signal processing framework.

## Features of the Tool

In the current stage of the Signal Processing tool, the signal can be accepted in the format of either an i32 vector or an f32 vector which is stored as an enum. There are then many operations that can be applied to this vector.

### 1) Math Operations 

The `Math` struct provides a collection of mathematical utility functions for analyzing and manipulating data
stored in the `RawData` enum. These functions cover various operations such as computing averages, normalizing
data, calculating z-scores, and finding minimum and maximum values.

The functions in this struct allow users to perform common data preprocessing tasks efficiently and accurately,
helping to enhance data analysis and feature extraction processes.

### 2) Data Resampling Operations

The `DataResampling` struct provides methods for resampling input data to a specified output frequency
by incrementing the index based on frequency ratios. It supports both raw floating-point and integer data.

### 3) Peak Finding Operations

The `PeakFinder` struct provides methods for detecting peaks in data vectors, facilitating peak analysis
and feature extraction. Peaks are significant local maxima in a signal that can carry important information
about underlying patterns and phenomena.

This struct offers flexibility in peak detection by allowing you to specify various parameters such as minimum
and maximum peak heights, minimum prominence, minimum distance between peaks, and minimum plateau size.
Peaks can be detected in both floating-point and integer data vectors.

### 4)  Fast Fourier Transform

The `Fft` struct provides methods to compute forward and inverse FFTs on real or integer data.
It utilizes an FFT planner to optimize memory usage and reduce setup time, allowing users to
conveniently transform data between time and frequency domains.

### 5 ) Windowing Operations

The `Windowing` struct provides a set of windowing functions for segmenting and preprocessing raw data frames
before applying further signal processing or analysis. Windowing is a common technique used in various fields
such as audio processing, image analysis, and data science.

Each windowing function takes a data `frame` and a specified `size` or `increment`, applies a specific windowing
function to the frame, and returns a vector representing the windowed frame. This process helps reduce artifacts
and improve the quality of subsequent processing, such as spectral analysis or feature extraction.

Supported windowing functions include:
 - Rectangular: Provides a basic windowing function with constant amplitude within the specified size.
 - Blackman: Applies a Blackman window to the frame, tapering the edges to reduce spectral leakage.
 - Hamming: Applies a Hamming window, which balances spectral leakage and side lobe attenuation.
 - Hann: Uses a Hann window to improve spectral leakage and reduce side lobes compared to rectangular windows.
 - Welch: Applies the Welch window, which reduces noise and provides better frequency domain representation.
 - Sine: Uses a sine window to emphasize a particular frequency component in the frame.
 - Triangular: Applies a triangular window for simple spectral analysis and smoothing.

### 6) FIR Digital Filters

The `FIR` module contains  a `Bandpass`,  `Highpass` , and a `Lowpass`struct for filtering the data frames within their
specified frequency ranges. Filtfilt is also available for these filters.

### 7) IIR Digital Filters

The `IIR` module contains  a `Bandpass`,  `Highpass` , `Lowpass`, and `Notch` struct for filtering the data frames within their
specified frequency ranges. Filtfilt is also available for these filters.

The `IIR` module also contains a `Moving Average Filter` struct
