use std::sync::{Arc, Mutex};

use audio::dasp_sample::ToSample;
use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use ringbuf::{HeapRb, Rb};

mod utils;
use utils::DrawExt;

const IMAGE_WIDTH: f32 = 720.0;
const IMAGE_HEIGHT: f32 = 615.0;
const TOTAL_PIXELS: u32 = IMAGE_WIDTH as u32 * IMAGE_HEIGHT as u32;
const DOT_SIZE: f32 = 1.0;

const SAMPLE_SIZE: usize = 512;
const RB_SIZE: usize = SAMPLE_SIZE * 10;

fn main() {
    nannou::app(model).update(update).run();
}

struct AppModel {
    in_stream: audio::Stream<RecorderModel>,
    scanner_index: Arc<Mutex<usize>>,
    rb: Arc<Mutex<HeapRb<f32>>>,
    display_index: usize,
}

struct RecorderModel {
    rb: Arc<Mutex<HeapRb<f32>>>,
    scanner_index: Arc<Mutex<usize>>,
}

fn model(app: &App) -> AppModel {
    app.new_window()
        .size(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();

    let scanner_index = Arc::new(Mutex::new(0));
    let display_index = 0;
    let rb = Arc::new(Mutex::new(HeapRb::<f32>::new(RB_SIZE)));

    let recorder_model = RecorderModel {
        rb: rb.clone(),
        scanner_index: scanner_index.clone(),
    };

    let audio_host = audio::Host::new();
    let in_stream = audio_host
        .new_input_stream(recorder_model)
        .channels(1)
        .frames_per_buffer(SAMPLE_SIZE)
        .capture(pass_in)
        .build()
        .unwrap();

    in_stream.play().unwrap();

    AppModel {
        in_stream,
        rb,
        scanner_index,
        display_index,
    }
}

fn pass_in(model: &mut RecorderModel, buffer: &Buffer) {
    // println!("pass in");
    for frame in buffer.frames() {
        for sample in frame {
            println!("sample: {}", sample);
            model.rb.lock().unwrap().push_overwrite(*sample);
        }
    }
    let mut scanner_index = model.scanner_index.lock().unwrap();
    *scanner_index = if *scanner_index + SAMPLE_SIZE < RB_SIZE {
        *scanner_index + SAMPLE_SIZE
    } else {
        0
    };
}

fn update(_app: &App, model: &mut AppModel, _update: Update) {
    // println!("current display index: {}", model.display_index);

    if (model.display_index as u32 + SAMPLE_SIZE as u32) > TOTAL_PIXELS {
        model.display_index = 0;
    } else {
        model.display_index += SAMPLE_SIZE;
    }
    // println!("new display index: {}", model.display_index);
}

fn view(app: &App, model: &AppModel, frame: Frame) {
    // println!("view");
    let draw = app.draw();
    let win = app.window_rect();
    let rb = model.rb.lock().unwrap();
    let scanner_index = model.scanner_index.lock().unwrap();
    // println!("scanner index: {}", *scanner_index);

    let mut brightest_value = -1.0;
    let mut darkest_value = 2.0;
    for (i, sample) in rb.iter().enumerate().skip(*scanner_index).take(SAMPLE_SIZE) {
        let index = model.display_index + i;
        let (x, y) = index_to_xy(index);
        let mapped_color = map_range(*sample, -1.0, 1.0, 0.0, 1.0);
        let color = gray(mapped_color);
        draw.point(x, y, color);

        if mapped_color > brightest_value {
            brightest_value = mapped_color;
        }
        if mapped_color < darkest_value {
            darkest_value = mapped_color;
        }
    }

    println!("brightest: {}, darkest: {}", brightest_value, darkest_value);
    draw.to_frame(app, &frame).unwrap();

    // println!("FPS: {}", app.fps());
}

fn index_to_xy(index: usize) -> (f32, f32) {
    let row = index / IMAGE_WIDTH as usize;
    let col = index % IMAGE_WIDTH as usize;
    let x = col as f32 * DOT_SIZE - IMAGE_WIDTH / 2.0 + DOT_SIZE / 2.0;
    let y = IMAGE_HEIGHT / 2.0 - row as f32 * DOT_SIZE - DOT_SIZE / 2.0;
    (x, y)
}
