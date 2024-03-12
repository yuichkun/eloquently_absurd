use std::{fs::File, io::BufWriter};

use nannou::prelude::*;
extern crate hound;
use hound::*;

const SAMPLE_RATE: u32 = 44100;
const MAX_AMP: f32 = i16::MAX as f32;

fn main() {
    println!("Hello, world!");

    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create("../noise.wav", spec).unwrap();
    generate_noise(&mut writer, 10);
    writer.finalize().unwrap();
}

fn generate_noise(writer: &mut WavWriter<BufWriter<File>>, sec: u32) {
    let num_samples = sec * SAMPLE_RATE;
    for t in (0..num_samples).map(|x| x as f32 / SAMPLE_RATE as f32) {
        let sample = random_range(-1000, 1000) as f32 / 1000.0;

        let fade_factor = 1.0 - (t / sec as f32);
        let amplitude = MAX_AMP * fade_factor;

        println!("amplitude: {}", amplitude);
        write_sample(writer, sample, amplitude);
    }
}

fn write_sample(writer: &mut WavWriter<BufWriter<File>>, sample: f32, amplitude: f32) {
    writer.write_sample((sample * amplitude) as i16).unwrap();
}
