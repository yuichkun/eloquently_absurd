use nannou::prelude::*;
use wgpu::*;

mod simple_shader;
use simple_shader::*;

struct Model {
    bind_group: BindGroup,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(512, 512).view(view).build().unwrap();

    let window = app.main_window();
    let device = window.device();

    let vs_desc = include_wgsl!("shaders/vs.wgsl");
    let fs_desc = include_wgsl!("shaders/fs.wgsl");

    let sample_count = window.msaa_samples();
    let SetupRenderPipelineOutput {
        bind_group,
        render_pipeline,
        vertex_buffer,
    } = setup_render_pipeline(SetupRenderPipelineParams {
        device,
        vs_desc,
        fs_desc,
        sample_count,
    });

    Model {
        bind_group,
        vertex_buffer,
        render_pipeline,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    render_shaders(
        &model.bind_group,
        &model.render_pipeline,
        &model.vertex_buffer,
        &frame,
    );
}
