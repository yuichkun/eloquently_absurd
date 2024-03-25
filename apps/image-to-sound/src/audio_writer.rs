use hound::*;

const SAMPLE_RATE: u32 = 44100;

pub fn write_audio_file(brightnessVec: Vec<u8>, filename: &str) {
    let duration = brightnessVec.len() as f32 / SAMPLE_RATE as f32;
    println!("Generating audio for {} seconds...", duration);
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create(filename, spec).unwrap();

    for val in brightnessVec {
        let sample = scale(val) as f32;
        writer
            .write_sample((sample * i16::MAX as f32) as i16)
            .unwrap();
    }

    writer.finalize().unwrap();
    println!("Done!");
}
fn scale(brightness: u8) -> f32 {
    return (brightness as f32 / 255.0) * 2.0 - 1.0;
}
