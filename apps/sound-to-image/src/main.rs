use nannou::prelude::*;
use wgpu::*;
mod fft;

mod simple_shader;
use simple_shader::*;

mod helpers;
use helpers::*;

mod recorder;
use recorder::{AppAudioBuffer, RecorderInStream};

mod ui;
use ui::AppUi;

pub const WIDTH: usize = 500;
pub const HEIGHT: usize = 500;

struct Model {
    #[allow(unused)]
    in_stream: RecorderInStream,

    rb: AppAudioBuffer,
    shader_settings: SetupRenderPipelineOutput,
    ui: AppUi,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Uniforms {
    time: f32,
    resolution: f32,
    amp: f32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .fullscreen()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let (rb, in_stream) = recorder::create();

    let window = app.main_window();
    let device = window.device();

    let vs_desc = include_wgsl!("shaders/vs.wgsl");
    let fs_desc = include_wgsl!("shaders/fs.wgsl");

    let sample_count = window.msaa_samples();

    let ui = ui::create_ui(&window);
    let uniforms = Uniforms {
        time: 0.0,
        resolution: ui.settings.resolution,
        amp: ui.settings.amp,
    };

    let shader_settings = setup_render_pipeline(SetupRenderPipelineParams {
        device,
        vs_desc,
        fs_desc,
        sample_count,
        uniforms: &uniforms,
    });

    Model {
        rb,
        in_stream,
        ui,
        shader_settings,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    simple_shader::update(app, model);

    // TODO: fix update ui
    ui::update_settings_ui(&mut model.ui);
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    render_shaders(model, &frame, app.main_window().device());

    // ui::show(model, &frame);
    app.show_fps(&frame);
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    ui::raw_event(_app, model, event);
}
