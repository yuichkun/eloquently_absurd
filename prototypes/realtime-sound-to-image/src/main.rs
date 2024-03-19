use std::sync::{Arc, Mutex};

use nannou::{prelude::*, state::keys};
use nannou_audio as audio;
use nannou_audio::Buffer;
use ringbuf::{HeapRb, Rb};

mod utils;
use utils::DrawExt;

const IMAGE_WIDTH: f32 = 720.0;
const IMAGE_HEIGHT: f32 = 615.0;
const DOT_SIZE: f32 = 1.0;

// 442,800
const RB_SIZE: usize = IMAGE_WIDTH as usize * IMAGE_HEIGHT as usize;
const SAMPLE_SIZE: usize = 512;

fn main() {
    nannou::app(model).update(update).run();
}

struct AppModel {
    #[allow(unused)]
    in_stream: audio::Stream<RecorderModel>,
    rb: Arc<Mutex<HeapRb<f32>>>,

    last_drawn_display_index: Mutex<usize>,
    how_many_pixels_to_draw_in_next_frame: usize,
}

struct RecorderModel {
    rb: Arc<Mutex<HeapRb<f32>>>,
}

fn model(app: &App) -> AppModel {
    app.new_window()
        .size(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let rb = Arc::new(Mutex::new(HeapRb::<f32>::new(RB_SIZE)));

    // for i in 0..RB_SIZE {
    //     let v = map_range(i as f32, 0.0, RB_SIZE as f32, -1.0, 1.0);
    //     rb.lock().unwrap().push(v).expect("Ring buffer overflow");
    // }

    let recorder_model = RecorderModel { rb: rb.clone() };

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
        last_drawn_display_index: Mutex::new(0),
        how_many_pixels_to_draw_in_next_frame: 0,
    }
}

fn pass_in(model: &mut RecorderModel, buffer: &Buffer) {
    let mut rb = model.rb.lock().unwrap();

    for frame in buffer.frames() {
        let capacity = rb.capacity();
        let rb_len = rb.len();
        if rb_len >= capacity {
            continue;
        }
        for sample in frame {
            rb.push(*sample).expect("Ring buffer overflow");
        }
    }
}

fn update(_app: &App, model: &mut AppModel, _update: Update) {
    println!("====update====");
    let rb = model.rb.lock().unwrap();
    let rb_len = rb.len();
    let last_drawn_display_index = model.last_drawn_display_index.lock().unwrap();
    println!("rb_len: {}", rb_len);
    println!("last_drawn_display_index: {}", last_drawn_display_index);

    model.how_many_pixels_to_draw_in_next_frame = rb_len - *last_drawn_display_index;

    println!(
        "how_many_pixels_to_draw_in_next_frame: {}",
        model.how_many_pixels_to_draw_in_next_frame
    );
}

fn view(app: &App, model: &AppModel, frame: Frame) {
    println!("====view====");
    let draw = app.draw();
    let rb = model.rb.lock().unwrap();
    let how_many_pixels_to_draw = model.how_many_pixels_to_draw_in_next_frame;
    let last_drawn_display_index;
    {
        let last_drawn_display_lock = model.last_drawn_display_index.lock().unwrap();
        last_drawn_display_index = *last_drawn_display_lock;
    }

    println!("last_drawn_display_index: {}", last_drawn_display_index);
    println!("how_many_pixels_to_draw: {}", how_many_pixels_to_draw);

    for (index, sample) in rb
        .iter()
        .enumerate()
        .skip(last_drawn_display_index)
        .take(how_many_pixels_to_draw)
    {
        let (x, y) = index_to_xy(index);
        let mapped_color = map_range(*sample, -1.0, 1.0, 0.0, 1.0);
        let color = gray(mapped_color);
        draw.point(x, y, color);
    }

    draw.to_frame(app, &frame).unwrap();

    *model.last_drawn_display_index.lock().unwrap() =
        last_drawn_display_index + how_many_pixels_to_draw;

    println!("FPS: {}", app.fps());
}

fn key_pressed(app: &App, model: &mut AppModel, key: Key) {
    match key {
        Key::Space => {
            reset_model(model);
        }
        _ => (),
    }
}

fn reset_model(model: &mut AppModel) {
    *model.last_drawn_display_index.lock().unwrap() = 0;
    model.how_many_pixels_to_draw_in_next_frame = 0;
    model.rb.lock().unwrap().clear();
}

fn index_to_xy(index: usize) -> (f32, f32) {
    let row = index / IMAGE_WIDTH as usize;
    let col = index % IMAGE_WIDTH as usize;
    let x = col as f32 * DOT_SIZE - IMAGE_WIDTH / 2.0 + DOT_SIZE / 2.0;
    let y = IMAGE_HEIGHT / 2.0 - row as f32 * DOT_SIZE - DOT_SIZE / 2.0;
    (x, y)
}
