use super::{Model, Uniforms};
use nannou::prelude::*;
use nannou::wgpu::{self};
use wgpu::*;

use super::RB_SIZE;

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}
const VERTICES: [Vertex; 4] = [
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
];

pub struct SetupRenderPipelineParams<'a> {
    pub device: &'a Device,
    pub vs_desc: ShaderModuleDescriptor<'a>,
    pub fs_desc: ShaderModuleDescriptor<'a>,
    pub sample_count: u32,
    pub uniforms: &'a Uniforms,
}

pub struct SetupRenderPipelineOutput {
    pub bind_group: BindGroup,
    pub render_pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,
    pub uniform_buffer: Buffer,
    pub audio_storage_buffer: Buffer,
}

pub fn setup_render_pipeline(params: SetupRenderPipelineParams) -> SetupRenderPipelineOutput {
    let SetupRenderPipelineParams {
        device,
        vs_desc,
        fs_desc,
        sample_count,
        uniforms,
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

    let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: uniforms_as_bytes(uniforms),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let audio_storage_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Audio Storage Buffer"),
        contents: bytemuck::cast_slice(&[0.0f32; RB_SIZE]),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });

    let storage_dynamic = false;
    let storage_readonly = true;
    // Create the render pipeline.
    let bind_group_layout = BindGroupLayoutBuilder::new()
        .storage_buffer(ShaderStages::FRAGMENT, storage_dynamic, storage_readonly)
        .uniform_buffer(ShaderStages::VERTEX | ShaderStages::FRAGMENT, false)
        .build(device);

    let audio_buffer_size = (RB_SIZE as usize * std::mem::size_of::<f32>()) as wgpu::BufferAddress;
    let buffer_size_bytes = std::num::NonZeroU64::new(audio_buffer_size).unwrap();

    let bind_group = BindGroupBuilder::new()
        .buffer_bytes(&audio_storage_buffer, 0, Some(buffer_size_bytes))
        .buffer::<Uniforms>(&uniform_buffer, 0..1)
        .build(device, &bind_group_layout);

    let pipeline_layout = create_pipeline_layout(device, None, &[&bind_group_layout], &[]);

    let render_pipeline = RenderPipelineBuilder::from_layout(&pipeline_layout, &vs_mod)
        .fragment_shader(&fs_mod)
        .color_format(Frame::TEXTURE_FORMAT)
        .add_vertex_buffer::<Vertex>(&vertex_attr_array![0 => Float32x2])
        .sample_count(sample_count)
        .primitive_topology(PrimitiveTopology::TriangleStrip)
        .build(device);

    SetupRenderPipelineOutput {
        bind_group,
        render_pipeline,
        vertex_buffer,
        uniform_buffer,
        audio_storage_buffer,
    }
}

pub fn render_shaders(
    model: &Model,
    bind_group: &BindGroup,
    render_pipeline: &RenderPipeline,
    vertex_buffer: &Buffer,
    frame: &Frame,
    device: &Device,
    uniforms: &Uniforms,
) {
    // Update the uniforms (rotate around the teapot).
    let uniforms_size = std::mem::size_of::<Uniforms>() as wgpu::BufferAddress;
    let uniforms_bytes = uniforms_as_bytes(&uniforms);
    let usage = wgpu::BufferUsages::COPY_SRC;
    let new_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: uniforms_bytes,
        usage,
    });

    let mut encoder = frame.command_encoder();
    encoder.copy_buffer_to_buffer(
        &new_uniform_buffer,
        0,
        &model.uniform_buffer,
        0,
        uniforms_size,
    );

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

fn uniforms_as_bytes(uniforms: &Uniforms) -> &[u8] {
    unsafe { wgpu::bytes::from(uniforms) }
}
