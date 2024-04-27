struct VertexOutput {
    @location(0) tex_coords: vec2<f32>,
    @builtin(position) out_pos: vec4<f32>,
};
struct Uniforms {
    time: f32,
    window_width: f32,
    window_height: f32, // New fields
};

@group(0) @binding(1)
var<uniform> uniforms: Uniforms;

@vertex
fn main(@location(0) pos: vec2<f32>) -> VertexOutput {
    // Calculate the NDC size based on the viewport size
    let ndc_width: f32 = 500.0 / uniforms.window_width * 2.0;
    let ndc_height: f32 = 500.0 / uniforms.window_height * 2.0;

    // Scale position to maintain the quad size in NDC
    let scaled_x: f32 = pos.x * ndc_width;
    let scaled_y: f32 = pos.y * ndc_height;

    // Center the quad in the middle of the screen
    let out_pos: vec4<f32> = vec4<f32>(scaled_x, scaled_y, 0.0, 1.0);

    // Adjust texture coordinates to range from 0.0 to 1.0
    let tex_coords: vec2<f32> = vec2((pos.x + 1.0) * 0.5, (1.0 - pos.y) * 0.5);

    return VertexOutput(tex_coords, out_pos);
}