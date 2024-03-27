struct Data {
    time: f32,
};


struct FragmentOutput {
    @location(0) f_color: vec4<f32>,
};

@group(0) @binding(0)
var tex: texture_2d<f32>;
@group(0) @binding(1)
var tex_sampler: sampler;
@group(0) @binding(2)
var<uniform> uniforms: Data;

@fragment
fn main(@location(0) tex_coords: vec2<f32>) -> FragmentOutput {
    let image_color: vec4<f32> = textureSample(tex, tex_sampler, tex_coords);
    let centered_uv = tex_coords - vec2<f32>(0.5, 0.5);

    var range = sin(uniforms.time * 1.5) * 0.5 + 0.5;
    range = max(range - 0.2, 0.0);
    var mask = distance(centered_uv, vec2<f32>(0.0, 0.0));
    mask = 1.0 - mask;
    mask = smoothstep(range, 1.0, mask);
    return FragmentOutput(vec4<f32>(vec3f(mask), 1.0) * image_color);
}
