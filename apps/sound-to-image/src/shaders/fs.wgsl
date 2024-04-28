struct Uniforms {
    time: f32,
    window_width: f32,
    window_height: f32, // New fields
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
    let index = u32(tex_coords.y * 500.0) * 500u + u32(tex_coords.x * 500.0);

    // Ensure the index does not go out of bounds
    let safeIndex = min(index, 249999u);

    // Get the sample value
    let sampleValue = audioData.samples[safeIndex];

    // Normalize the sample value to (0.0 to 1.0) for color mapping
    let colorValue = (sampleValue + 1.0) * 0.5;

    // Adjust contrast
    let contrastFactor: f32 = 2.2; // Example contrast factor, adjust as needed
    let adjustedColorValue = (colorValue - 0.5) * contrastFactor + 0.5;

    // Ensure the color value remains in the 0.0 to 1.0 range
    let clampedColorValue = clamp(adjustedColorValue, 0.0, 1.0);

    // Create the color vector
    var color: vec3<f32> = vec3<f32>(clampedColorValue, clampedColorValue, clampedColorValue);

    // Return the color as the fragment output
    return FragmentOutput(vec4<f32>(color, 1.0));
}