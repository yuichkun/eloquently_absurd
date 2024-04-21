use hound::*;
use std::io::Cursor;

const SAMPLE_RATE: u32 = 44100;

// Embed the start_signal.wav file directly into the binary
const START_SIGNAL: &'static [u8] = include_bytes!("../start_signal.wav");

pub fn write_audio_file(brightness_vec: Vec<u8>, filename: &str) {
    // Use a Cursor to read the embedded start_signal.wav
    let mut start_signal_reader =
        WavReader::new(Cursor::new(START_SIGNAL)).expect("Failed to read start_signal.wav");

    // Prepare the output file writer with the same specifications
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut writer = WavWriter::create(filename, spec).expect("Failed to create output file");

    // Write the start signal samples to the output file
    for sample in start_signal_reader.samples::<i16>() {
        let sample_value = sample.expect("Failed to read sample from start_signal.wav");
        writer
            .write_sample(sample_value)
            .expect("Failed to write start signal sample");
    }

    // Now, process and write the brightness_vec samples as before
    for val in brightness_vec {
        let sample = scale(val) as f32;
        writer
            .write_sample((sample * i16::MAX as f32) as i16)
            .expect("Failed to write brightness sample");
    }

    writer
        .finalize()
        .expect("Failed to finalize the output file");
    println!("Done!");
}

fn scale(brightness: u8) -> f32 {
    (brightness as f32 / 255.0) * 2.0 - 1.0
}
