use nannou::image;
use nannou::prelude::*;

fn main() {
    nannou::sketch(view_2).run();
}

fn view_2(app: &App, frame: Frame) {
    frame.clear(BLACK);

    let window = app.main_window();
    let win = window.rect();
    let width = win.w() as u32;
    let height = win.h() as u32;

    let texture = wgpu::TextureBuilder::new()
        .size([width, height])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(window.device());
    let image = image::ImageBuffer::from_fn(width, height, |x, y| {
        let normalized_x = x as f32 / width as f32;
        let normalized_y = y as f32 / height as f32;
        let r: u8 = (normalized_x * 255.0).round() as u8;
        let g: u8 = (normalized_y * 255.0).round() as u8;
        return nannou::image::Rgba([r, g, 0, std::u8::MAX]);
    });
    let flat_samples = image.as_flat_samples();
    texture.upload_data(
        app.main_window().device(),
        &mut *frame.command_encoder(),
        &flat_samples.as_slice(),
    );

    let draw = app.draw();
    draw.texture(&texture);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
