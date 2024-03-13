use nannou::image;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    texture: wgpu::Texture,
    b: u8,
}

fn model(app: &App) -> Model {
    app.new_window().size(300, 300).view(view).build().unwrap();
    let window = app.main_window();
    let win = app.window_rect();
    let width = win.w() as u32;
    let height = win.h() as u32;

    let texture = wgpu::TextureBuilder::new()
        .size([width, height])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(window.device());
    Model { texture, b: 0 }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let win = app.window_rect();
    let width = win.w() as u32;
    let height = win.h() as u32;

    let image = image::ImageBuffer::from_fn(width, height, |x, y| {
        let normalized_x = x as f32 / width as f32;
        let normalized_y = y as f32 / height as f32;
        let r: u8 = (normalized_x * 255.0).round() as u8;
        let g: u8 = (normalized_y * 255.0).round() as u8;
        nannou::image::Rgba([r, g, model.b, std::u8::MAX])
    });
    let flat_samples = image.as_flat_samples();
    model.texture.upload_data(
        app.main_window().device(),
        &mut frame.command_encoder(),
        &flat_samples.as_slice(),
    );
    let draw = app.draw();
    draw.texture(&model.texture);
    draw.to_frame(app, &frame).unwrap();
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.b == std::u8::MAX {
        model.b = 0;
    } else {
        model.b += 3;
    }
    println!("update b: {}", model.b);
}
