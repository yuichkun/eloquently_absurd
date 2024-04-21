use super::Model;
use ringbuf::Rb;
use rustfft::{num_complex::Complex, FftPlanner};

fn detect_start_signal(samples: Vec<f32>) -> bool {
    let sample_len = samples.len();
    // Create an FFT planner
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(sample_len);

    // Convert samples to complex numbers (real part is the sample, imaginary part is 0)
    let mut buffer: Vec<_> = samples
        .into_iter()
        .map(|s| Complex { re: s, im: 0.0 })
        .collect();

    // Perform the FFT
    fft.process(&mut buffer);

    let target_frequencies = [(200.0, 10.0), (16000.0, 15.0)];
    let sample_rate = 44100; // Your audio sample rate

    for &(freq, thresh) in &target_frequencies {
        let bin = (freq / sample_rate as f32) * sample_len as f32;
        let amplitude = buffer[bin as usize].norm(); // Simplified, consider using a range around `bin`
        if amplitude < thresh {
            return false; // If any amplitude is below its threshold, return false
        }
    }

    true
}

pub fn update(model: &mut Model) {
    let mut rb = model.rb.lock().unwrap();
    let samples: Vec<f32> = rb.iter().copied().collect();
    let sample_count = 512;
    let samples_to_consider = if samples.len() > sample_count {
        &samples[(samples.len() - sample_count)..]
    } else {
        &samples[..]
    };
    if detect_start_signal(samples_to_consider.to_vec()) {
        println!("Start signal detected");
        rb.clear();
    }
}
