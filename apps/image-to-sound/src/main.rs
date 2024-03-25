use clap::Parser;
use image::GenericImageView;
mod audio_writer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    image_path: String,
    file_name: String,
}

fn main() {
    println!("IMAGE TO SOUND CONVERTER");
    let args = Cli::parse();
    println!("image_path: {:?}", args.image_path);
    println!("file_name: {:?}", args.file_name);
    let img = image::open(args.image_path).unwrap();
    println!("dimensions {:?}", img.dimensions());
    let mut brightnessVec: Vec<u8> = Vec::new();

    for (_, _, pixel) in img.pixels() {
        let rgba = pixel.0;
        let brightness = (rgba[0] as u32 + rgba[1] as u32 + rgba[2] as u32) / 3;
        brightnessVec.push(brightness as u8);
    }

    audio_writer::write_audio_file(brightnessVec, &args.file_name);
}
