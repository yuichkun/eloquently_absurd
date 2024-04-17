struct Uniforms {
    time: f32,
    resolution: f32,
    amp: f32, // Added amp parameter
};
struct AudioData {
    samples: array<f32, 250000>,
};

struct FragmentOutput {
    @location(0) f_color: vec4<f32>,
};

@group(0) @binding(0) var<storage, read> audioData: AudioData;
@group(0) @binding(1)
var<uniform> uniforms: Uniforms;

@fragment
fn main(@location(0) tex_coords: vec2<f32>) -> FragmentOutput {
    // Calculate the 1D index from the 2D texture coordinates
    // Assuming tex_coords are normalized (0.0 to 1.0)
    let index = u32(tex_coords.y * 500.0) * 500u + u32(tex_coords.x * 500.0);

    // Ensure the index does not go out of bounds
    let safeIndex = min(index, 249999u);

    // Get the sample value
    let sampleValue = audioData.samples[safeIndex];

    // Map the sample value (-1.0 to 1.0) directly to a grayscale color
    // Normalize the sample value to (0.0 to 1.0) for color mapping
    let colorValue = (sampleValue + 1.0) * 0.5;

   // Create the color vector
    var color: vec3<f32> = vec3<f32>(colorValue, colorValue, colorValue);
    // Return the color as the fragment output
    return FragmentOutput(vec4<f32>(color, 1.0));
}