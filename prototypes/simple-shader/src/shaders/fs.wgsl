struct Data {
    time: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Data;

@fragment
fn main(
    @location(0) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    let centered_uv = uv - vec2<f32>(0.5, 0.5);
    let d = length(centered_uv);
    let s = step(0.3, d);
    let amp = sin(uniforms.time * 10.0) * 0.5 + 0.5;
    let color = vec3<f32>(s * amp);

    return vec4<f32>(color, 1.0);
}