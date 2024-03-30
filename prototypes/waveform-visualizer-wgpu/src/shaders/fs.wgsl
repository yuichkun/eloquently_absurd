struct Uniforms {
    time: f32,
    resolution: f32,
};
struct AudioData {
    samples: array<f32, 1024>,
};


struct FragmentOutput {
    @location(0) f_color: vec4<f32>,
};

@group(0) @binding(0) var<storage, read> audioData: AudioData;
@group(0) @binding(1)
var<uniform> uniforms: Uniforms;

@fragment
fn main(@location(0) tex_coords: vec2<f32>) -> FragmentOutput {
    // Map tex_coords.x to sample index
    let sampleIndex = u32(tex_coords.x * 1024.0);

    // Get the sample value
    let sample = audioData.samples[sampleIndex];

    // Map the sample value (-1.0 to 1.0) to a vertical position
    let verticalPosition = (sample + 1.0) / 2.0; // Now 0.0 to 1.0

     // Determine if the current fragment is close to the vertical position
    let isCloseToSample = abs(tex_coords.y - verticalPosition) < uniforms.resolution; // Adjust the threshold as needed


    // Initialize color as black
    var color: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

    // Conditionally set color to white if close to the sample
    if (isCloseToSample) {
        color = vec3<f32>(1.0, 1.0, 1.0);
    }

    return FragmentOutput(vec4<f32>(color, 1.0));

}