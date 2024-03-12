use std::{env, fs::File, io::BufWriter};

use nannou::prelude::*;
extern crate hound;
use hound::*;

const SAMPLE_RATE: u32 = 44100;
const MAX_AMP: f32 = i16::MAX as f32;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <seconds> <export_path>", args[0]);
        std::process::exit(1);
    }
    let sec: u32 = args[1]
        .parse()
        .expect("Please provide a valid number for seconds");
    let export_path = &args[2];

    println!("Generating noise for {} seconds...", sec);

    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create(export_path, spec).unwrap();
    generate_noise(&mut writer, sec);
    writer.finalize().unwrap();
    println!("Done!");
}

fn generate_noise(writer: &mut WavWriter<BufWriter<File>>, sec: u32) {
    let num_samples = sec * SAMPLE_RATE;
    for t in (0..num_samples).map(|x| x as f32 / SAMPLE_RATE as f32) {
        let sample = random_range(-1000, 1000) as f32 / 1000.0;

        let fade_factor = 1.0 - (t / sec as f32);
        let amplitude = MAX_AMP * fade_factor;
        write_sample(writer, sample, amplitude);
    }
}

fn write_sample(writer: &mut WavWriter<BufWriter<File>>, sample: f32, amplitude: f32) {
    writer.write_sample((sample * amplitude) as i16).unwrap();
}
