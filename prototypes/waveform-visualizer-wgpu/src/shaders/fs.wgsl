struct Uniforms {
    time: f32,
    resolution: f32,
    amp: f32, // Added amp parameter
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
    // Calculate the exact position in the samples array
    let exactSampleIndex = tex_coords.x * 1024.0;
    // Determine the indices of the two samples to interpolate between
    let sampleIndex1 = u32(exactSampleIndex);
    let sampleIndex2 = min(sampleIndex1 + 1u, 1023u); // Ensure we don't go out of bounds

    // Get the two sample values
    let sample1 = audioData.samples[sampleIndex1];
    let sample2 = audioData.samples[sampleIndex2];

    // Calculate the interpolation factor (how far we are between the two samples)
    let t = fract(exactSampleIndex);

    // Linearly interpolate between the two sample values
    let interpolatedSample = mix(sample1, sample2, t);

    // Apply the amp parameter to the interpolated sample
    let ampedSample = interpolatedSample * uniforms.amp;

    // Map the amped sample value (-1.0 to 1.0) to a vertical position
    let verticalPosition = (ampedSample + 1.0) / 2.0; // Now 0.0 to 1.0

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