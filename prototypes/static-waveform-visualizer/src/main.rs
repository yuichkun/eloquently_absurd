use hound;
use nannou::prelude::*;

const RB_SIZE: usize = 1024;

fn main() {
    nannou::app(model).run();
}

struct AppModel {
    buffer: Vec<i16>,
    samples_to_draw: usize,
    offset: usize,
}
fn mouse_wheel(_app: &App, model: &mut AppModel, delta: MouseScrollDelta, _phase: TouchPhase) {
    let (zoom_amount, scroll_amount) = match delta {
        MouseScrollDelta::LineDelta(x, y) => (y as i32 * 10, x as i32 * 10), // Adjust the multiplier as needed
        MouseScrollDelta::PixelDelta(pos) => (pos.y as i32, pos.x as i32),
    };
    println!("Zoom: {}, Scroll: {}", zoom_amount, scroll_amount);

    // Zoom in or out
    if zoom_amount > 0 {
        model.samples_to_draw =
            (model.samples_to_draw + zoom_amount as usize).min(model.buffer.len());
    } else {
        model.samples_to_draw = model
            .samples_to_draw
            .checked_sub(zoom_amount.abs() as usize)
            .unwrap_or(0);
    }

    // Scroll right or left
    if scroll_amount > 0 {
        model.offset = model
            .offset
            .checked_sub(scroll_amount.abs() as usize)
            .unwrap_or(0);
    } else {
        model.offset = (model.offset + scroll_amount.abs() as usize)
            .min(model.buffer.len() - model.samples_to_draw);
    }
}

fn model(app: &App) -> AppModel {
    app.new_window()
        .size(RB_SIZE as u32, RB_SIZE as u32)
        .mouse_wheel(mouse_wheel)
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();

    let mut reader = hound::WavReader::open("noise.wav").unwrap();

    let duration = reader.duration();
    let len = reader.len();
    println!("Duration: {}, len: {}", duration, len);

    let buffer: Vec<i16> = reader.samples::<i16>().filter_map(Result::ok).collect();

    AppModel {
        buffer,
        samples_to_draw: 1000,
        offset: 0,
    }
}

fn view(app: &App, model: &AppModel, frame: Frame) {
    let draw = app.draw();
    frame.clear(BLACK);
    let win = app.window_rect();

    let mut last_point = win.mid_left();
    let buffer = &model.buffer;

    let samples_to_draw = model.samples_to_draw;

    buffer
        .iter()
        .enumerate()
        .skip(model.offset)
        .take(samples_to_draw)
        .for_each(|(index, sample)| {
            let half_h = win.h() / 2.0;
            let signal = map_range(*sample, i16::MIN, i16::MAX, half_h * -1.0, half_h);
            let x_pos = map_range(
                index,
                model.offset,
                model.offset + samples_to_draw,
                win.left(),
                win.right(),
            );
            let current_point = pt2(x_pos, signal);

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

fn key_pressed(_app: &App, model: &mut AppModel, key: Key) {
    let offset_amount = 1;
    let draw_amount = 10;
    match key {
        Key::Right => {
            model.offset = (model.offset + offset_amount).min(model.buffer.len());
        }
        Key::Left => {
            model.offset = model.offset.checked_sub(offset_amount).unwrap_or(0);
        }
        Key::Up => {
            model.samples_to_draw = (model.samples_to_draw + draw_amount).min(model.buffer.len());
        }
        Key::Down => {
            model.samples_to_draw = model.samples_to_draw.checked_sub(draw_amount).unwrap_or(0);
        }
        _ => {}
    }
}
