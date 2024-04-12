use bytemuck;
use nannou::prelude::*;
use wgpu::*;

mod simple_shader;
use simple_shader::*;

mod helpers;
use helpers::*;

mod recorder;
use recorder::{AppAudioBuffer, RecorderInStream, RB_SIZE};

mod ui;
use ui::AppUi;

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
        .size(RB_SIZE as u32, RB_SIZE as u32)
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
    model.shader_settings.uniforms.time = app.time;
    // Create a command encoder
    let mut encoder =
        app.main_window()
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Audio Storage Buffer Update Encoder"),
            });

    // Update the audio storage buffer with the latest samples from the ring buffer
    update_audio_storage_buffer(
        app.main_window().device(),
        &mut encoder,
        &model.shader_settings.audio_storage_buffer,
        &model.rb,
    );

    // Submit the commands to the GPU
    app.main_window()
        .queue()
        .submit(std::iter::once(encoder.finish()));

    // TODO: fix update ui
    ui::update_settings_ui(&mut model.ui);
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    render_shaders(
        model,
        &model.shader_settings.bind_group,
        &model.shader_settings.render_pipeline,
        &model.shader_settings.vertex_buffer,
        &frame,
        app.main_window().device(),
        &model.shader_settings.uniforms,
    );

    model.ui.egui.draw_to_frame(&frame).unwrap();
    app.show_fps(&frame);
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.ui.egui.handle_raw_event(event);
}

fn update_audio_storage_buffer(
    device: &Device,
    encoder: &mut CommandEncoder,
    audio_storage_buffer: &Buffer,
    rb: &AppAudioBuffer,
) {
    // Lock the ring buffer and collect the samples into a Vec
    let samples: Vec<f32> = recorder::collect_samples(rb);

    // Create a temporary buffer with the new audio samples
    let temp_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Temp Audio Buffer"),
        contents: bytemuck::cast_slice(&samples),
        usage: wgpu::BufferUsages::COPY_SRC,
    });

    // Calculate the size of the data to copy
    let data_size = (samples.len() * std::mem::size_of::<f32>()) as BufferAddress;

    // Copy the data from the temporary buffer to the audio_storage_buffer
    encoder.copy_buffer_to_buffer(&temp_buffer, 0, audio_storage_buffer, 0, data_size);
}
