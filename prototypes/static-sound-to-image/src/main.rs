use hound;
use nannou::prelude::*;

mod utils;
use utils::DrawExt;

const DOT_SIZE: f32 = 1.0;
const IMAGE_WIDTH: f32 = 720.0;
const IMAGE_HEIGHT: f32 = 615.0;

fn main() {
    nannou::app(model).run();
}

struct AppModel {
    buffer: Vec<i16>,
}

fn model(app: &App) -> AppModel {
    app.new_window()
        .size(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();

    let mut reader = hound::WavReader::open("out.wav").unwrap();

    let duration = reader.duration();
    let len = reader.len();
    println!("Duration: {}, len: {}", duration, len);

    let buffer: Vec<i16> = reader.samples::<i16>().filter_map(Result::ok).collect();

    AppModel { buffer }
}

fn view(app: &App, model: &AppModel, frame: Frame) {
    let draw = app.draw();
    frame.clear(BLACK);
    let win = app.window_rect();
    draw.point(win.w() / 2.0, win.h() / 2.0, gray(1.0));

    for (index, sample) in model.buffer.iter().enumerate() {
        let (x, y) = index_to_xy(index);
        let color = gray(map_range(*sample, i16::MIN, i16::MAX, 0.0, 1.0));
        draw.point(x, y, color);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn index_to_xy(index: usize) -> (f32, f32) {
    let row = index / IMAGE_WIDTH as usize;
    let col = index % IMAGE_WIDTH as usize;
    let x = col as f32 * DOT_SIZE - IMAGE_WIDTH / 2.0 + DOT_SIZE / 2.0;
    let y = IMAGE_HEIGHT / 2.0 - row as f32 * DOT_SIZE - DOT_SIZE / 2.0;
    (x, y)
}
