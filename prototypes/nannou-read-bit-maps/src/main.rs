use image::GenericImageView;
mod audio_writer;

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("/Users/yuichkun/Desktop/test.jpeg").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    let mut brightnessVec: Vec<u8> = Vec::new();

    for (_, _, pixel) in img.pixels() {
        let rgba = pixel.0; // Extract the inner RGBA value
        let brightness = (rgba[0] as u32 + rgba[1] as u32 + rgba[2] as u32) / 3;
        // println!("pixel at ({}, {}): {:?}", x, y, rgba);
        // println!("brightness: {}", brightness);
        brightnessVec.push(brightness as u8);
    }

    println!("brightnessVec: {:?}", brightnessVec);

    audio_writer::write_audio_file(brightnessVec)
}
