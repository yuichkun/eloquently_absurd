use nannou::image;
use nannou::image::GenericImageView;
use nannou::noise::{MultiFractal, NoiseFn, Seedable};
use nannou::prelude::*;
use nannou::prelude::*;
use nannou::rand::rngs::SmallRng;
use nannou::rand::{Rng, SeedableRng};
use wgpu::*;

mod simple_shader;
use simple_shader::*;

struct Model {
    bind_group: BindGroup,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    uniforms: Uniforms,
    uniform_buffer: Buffer,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Uniforms {
    time: f32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(512, 512).view(view).build().unwrap();
    let window = app.main_window();
    let win = window.rect();

    let mut rng = SmallRng::seed_from_u64(43);
    let image = image::ImageBuffer::from_fn(win.w() as u32, win.h() as u32, |_x, _y| {
        let r: u8 = rng.gen_range(0..std::u8::MAX);
        nannou::image::Rgba([r, r, r, std::u8::MAX])
    });
    let texture = Texture::load_from_image_buffer(
        window.device(),
        window.queue(),
        TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
        &image,
    );
    let texture_view = texture.view().build();

    let window = app.main_window();
    let device = window.device();

    let vs_desc = include_wgsl!("shaders/vs.wgsl");
    let fs_desc = include_wgsl!("shaders/fs.wgsl");

    let sample_count = window.msaa_samples();

    let uniforms = Uniforms { time: 0.0 };

    let SetupRenderPipelineOutput {
        bind_group,
        render_pipeline,
        vertex_buffer,
        uniform_buffer,
    } = setup_render_pipeline(SetupRenderPipelineParams {
        device,
        vs_desc,
        fs_desc,
        sample_count,
        uniforms: &uniforms,
        texture_view: &texture_view,
    });

    Model {
        bind_group,
        vertex_buffer,
        render_pipeline,
        uniforms,
        uniform_buffer,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.uniforms.time = app.time;
}

fn view(app: &App, model: &Model, frame: Frame) {
    render_shaders(
        model,
        &model.bind_group,
        &model.render_pipeline,
        &model.vertex_buffer,
        &frame,
        app.main_window().device(),
        &model.uniforms,
    );
}
