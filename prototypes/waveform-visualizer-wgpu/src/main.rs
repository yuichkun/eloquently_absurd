use bytemuck;
use nannou::prelude::*;
use nannou_audio as audio;
use nannou_egui::{self, egui, Egui};
use ringbuf::{Rb, StaticRb};
use std::sync::{Arc, Mutex};
use wgpu::*;

mod simple_shader;
use simple_shader::*;

const RB_SIZE: usize = 1024;

struct Settings {
    resolution: f32,
}

struct Model {
    #[allow(unused)]
    in_stream: audio::Stream<RecorderModel>,
    bind_group: BindGroup,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    uniforms: Uniforms,
    uniform_buffer: Buffer,
    rb: Arc<Mutex<StaticRb<f32, RB_SIZE>>>,
    audio_storage_buffer: Buffer,
    egui: Egui,
    settings: Settings,
}

struct RecorderModel {
    rb: Arc<Mutex<StaticRb<f32, RB_SIZE>>>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Uniforms {
    time: f32,
    resolution: f32,
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

    let rb = Arc::new(Mutex::new(StaticRb::<f32, RB_SIZE>::default()));
    let input_rb = rb.clone();
    let recorder_model = RecorderModel { rb: input_rb };
    let audio_host = audio::Host::new();
    let in_stream = audio_host
        .new_input_stream(recorder_model)
        .capture(pass_in)
        .build()
        .unwrap();

    in_stream.play().unwrap();

    let window = app.main_window();
    let device = window.device();

    let vs_desc = include_wgsl!("shaders/vs.wgsl");
    let fs_desc = include_wgsl!("shaders/fs.wgsl");

    let sample_count = window.msaa_samples();

    let settings = Settings {
        resolution: 0.00046,
    };

    let uniforms = Uniforms {
        time: 0.0,
        resolution: settings.resolution,
    };

    let SetupRenderPipelineOutput {
        bind_group,
        render_pipeline,
        vertex_buffer,
        uniform_buffer,
        audio_storage_buffer,
    } = setup_render_pipeline(SetupRenderPipelineParams {
        device,
        vs_desc,
        fs_desc,
        sample_count,
        uniforms: &uniforms,
    });

    let egui = Egui::from_window(&window);

    Model {
        rb,
        in_stream,
        bind_group,
        vertex_buffer,
        render_pipeline,
        uniforms,
        uniform_buffer,
        audio_storage_buffer,
        egui,
        settings,
    }
}

fn pass_in(model: &mut RecorderModel, buffer: &nannou_audio::Buffer) {
    for frame in buffer.frames() {
        for sample in frame {
            model.rb.lock().unwrap().push_overwrite(*sample);
        }
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.uniforms.time = app.time;
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
        &model.audio_storage_buffer,
        &model.rb,
    );

    // Submit the commands to the GPU
    app.main_window()
        .queue()
        .submit(std::iter::once(encoder.finish()));

    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        // Resolution slider
        ui.label("Resolution:");
        if ui
            .add(egui::Slider::new(&mut settings.resolution, 0.0001..=0.0005))
            .changed()
        {
            // Update the uniform value when the slider changes
            model.uniforms.resolution = settings.resolution;
        }
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    render_shaders(
        model,
        &model.bind_group,
        &model.render_pipeline,
        &model.vertex_buffer,
        &frame,
        app.main_window().device(),
        &model.uniforms,
    );

    model.egui.draw_to_frame(&frame).unwrap();
    show_fps(app, &frame);
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn show_fps(app: &App, frame: &Frame) {
    let fps = app.fps();
    let fps_text = format!("FPS: {:.2}", fps);

    // Position the text in the top right corner
    let draw = app.draw();
    draw.text(&fps_text)
        .color(WHITE)
        .font_size(16)
        .xy(app.window_rect().top_right() + vec2(-60.0, -20.0)); // Adjust the position as needed
    draw.to_frame(app, &frame).unwrap();
}

fn update_audio_storage_buffer(
    device: &Device,
    encoder: &mut CommandEncoder,
    audio_storage_buffer: &Buffer,
    rb: &Arc<Mutex<StaticRb<f32, RB_SIZE>>>,
) {
    // Lock the ring buffer and collect the samples into a Vec
    let samples: Vec<f32> = {
        let rb = rb.lock().unwrap();
        rb.iter().copied().collect()
    };

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
