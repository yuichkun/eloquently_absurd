use nannou::prelude::*;
use nannou::wgpu::{self};
use wgpu::*;

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}
const VERTICES: [Vertex; 6] = [
    Vertex {
        position: [-1.0, -1.0],
    }, // Bottom left
    Vertex {
        position: [1.0, -1.0],
    }, // Bottom right
    Vertex {
        position: [-1.0, 1.0],
    }, // Top left
    Vertex {
        position: [1.0, -1.0],
    }, // Bottom right
    Vertex {
        position: [1.0, 1.0],
    }, // Top right
    Vertex {
        position: [-1.0, 1.0],
    }, // Top left
];

pub struct SetupRenderPipelineParams<'a> {
    pub device: &'a Device,
    pub vs_desc: ShaderModuleDescriptor<'a>,
    pub fs_desc: ShaderModuleDescriptor<'a>,
    pub sample_count: u32,
}

pub struct SetupRenderPipelineOutput {
    pub bind_group: BindGroup,
    pub render_pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,
}

pub fn setup_render_pipeline(params: SetupRenderPipelineParams) -> SetupRenderPipelineOutput {
    let SetupRenderPipelineParams {
        device,
        vs_desc,
        fs_desc,
        sample_count,
    } = params;

    let vs_mod = device.create_shader_module(vs_desc);
    let fs_mod = device.create_shader_module(fs_desc);
    // Create the vertex buffer.
    let vertices_bytes = vertices_as_bytes(&VERTICES[..]);
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: vertices_bytes,
        usage: BufferUsages::VERTEX,
    });
    // Create the render pipeline.
    let bind_group_layout = BindGroupLayoutBuilder::new().build(device);
    let bind_group = BindGroupBuilder::new().build(device, &bind_group_layout);
    let pipeline_layout = create_pipeline_layout(device, None, &[&bind_group_layout], &[]);
    let render_pipeline = RenderPipelineBuilder::from_layout(&pipeline_layout, &vs_mod)
        .fragment_shader(&fs_mod)
        .color_format(Frame::TEXTURE_FORMAT)
        .add_vertex_buffer::<Vertex>(&vertex_attr_array![0 => Float32x2])
        .sample_count(sample_count)
        .build(device);

    SetupRenderPipelineOutput {
        bind_group,
        render_pipeline,
        vertex_buffer,
    }
}

pub fn render_shaders(
    bind_group: &BindGroup,
    render_pipeline: &RenderPipeline,
    vertex_buffer: &Buffer,
    frame: &Frame,
) {
    let mut encoder = frame.command_encoder();

    // The render pass can be thought of a single large command consisting of sub commands. Here we
    // begin a render pass that outputs to the frame's texture. Then we add sub-commands for
    // setting the bind group, render pipeline, vertex buffers and then finally drawing.
    let mut render_pass = RenderPassBuilder::new()
        .color_attachment(frame.texture_view(), |color| color)
        .begin(&mut encoder);
    render_pass.set_bind_group(0, bind_group, &[]);
    render_pass.set_pipeline(render_pipeline);
    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

    // We want to draw the whole range of vertices, and we're only drawing one instance of them.
    let vertex_range = 0..VERTICES.len() as u32;
    let instance_range = 0..1;
    render_pass.draw(vertex_range, instance_range);
}

// See the `nannou::wgpu::bytes` documentation for why this is necessary.
fn vertices_as_bytes(data: &[Vertex]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}
