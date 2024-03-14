use std::sync::{Arc, Mutex};

use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use ringbuf::{Rb, StaticRb};

const RB_SIZE: usize = 1024;
const AMP: f32 = 4.24;

fn main() {
    nannou::app(model).run();
}

struct AppModel {
    in_stream: audio::Stream<RecorderModel>,
    rb: Arc<Mutex<StaticRb<f32, RB_SIZE>>>,
}

struct RecorderModel {
    rb: Arc<Mutex<StaticRb<f32, RB_SIZE>>>,
}

fn model(app: &App) -> AppModel {
    app.new_window()
        .size(RB_SIZE as u32, RB_SIZE as u32)
        .view(view)
        .build()
        .unwrap();

    let mut rb = Arc::new(Mutex::new(StaticRb::<f32, RB_SIZE>::default()));

    let input_rb = rb.clone();
    let recorder_model = RecorderModel { rb: input_rb };

    let audio_host = audio::Host::new();
    let in_stream = audio_host
        .new_input_stream(recorder_model)
        .capture(pass_in)
        .build()
        .unwrap();

    in_stream.play().unwrap();

    AppModel { in_stream, rb }
}

fn pass_in(model: &mut RecorderModel, buffer: &Buffer) {
    for frame in buffer.frames() {
        for sample in frame {
            model.rb.lock().unwrap().push_overwrite(*sample);
        }
    }
}

fn view(app: &App, model: &AppModel, frame: Frame) {
    let draw = app.draw();
    frame.clear(BLACK);
    let rb = model.rb.lock().unwrap();
    let win = app.window_rect();

    let mut last_point = win.mid_left();

    rb.iter().enumerate().for_each(|(index, sample)| {
        let half_h = win.h() / 2.0;
        let signal = map_range(*sample, -1.0, 1.0, half_h * -1.0, half_h);
        let x_pos = map_range(index, 0, rb.len(), win.left(), win.right());
        let current_point = pt2(x_pos, signal * AMP);

        draw.line()
            .start(last_point)
            .end(current_point)
            .weight(0.3)
            .caps_round()
            .color(WHITE);

        last_point = current_point;
    });

    draw.to_frame(app, &frame).unwrap();
}
