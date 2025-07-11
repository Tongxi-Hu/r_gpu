struct Uniform {
    color: vec4<f32>,
    resolution: vec2<f32>,
    scaling: f32,
    rotation: f32,
    translation: vec2<f32>,
}

struct Input {
    @location(0) position: vec2<f32>,
}

struct Inter {
    @builtin(position) position: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> uni: Uniform;

// generate 2d scaling matrix
fn scaling_2d(scaling: f32) -> mat3x3<f32> {
    return mat3x3<f32>(vec3<f32>(scaling, 0, 0), vec3<f32>(0, scaling, 0), vec3<f32>(0, 0, 1));
}

// generate 2d rotation matrix
fn rotation_2d(rotation_deg: f32) -> mat3x3<f32> {
    const PI: f32 = 3.141592653589793238462643;
    return mat3x3<f32>(vec3<f32>(cos(PI * rotation_deg / 180.0), - sin(PI * rotation_deg / 180.0), 0), vec3<f32>(sin(PI * rotation_deg / 180.0), cos(PI * rotation_deg / 180.0), 0), vec3<f32>(0, 0, 1));
}

// generate 2d translation matrix
fn translation_2d(translation: vec2<f32>) -> mat3x3<f32> {
    return mat3x3<f32>(vec3<f32>(1, 0, uni.translation.x), vec3<f32>(0, 1, uni.translation.y), vec3<f32>(0, 0, 1));
}

@vertex
fn vs_main(in: Input) -> Inter {
    // physical space transformation 3*3 mat
    let scaling = scaling_2d(uni.scaling);
    let rotation = rotation_2d(uni.rotation);
    let translation = translation_2d(uni.translation);
    let transformed = vec3<f32>(in.position.x, in.position.y, 1) * (scaling * rotation * translation);
    // view space transformation
    let zero_to_one = transformed.xy / uni.resolution;
    let zero_to_two = zero_to_one * 2.0;
    let flipped = zero_to_two - 1.0;
    let clipped_space = flipped * vec2<f32>(1, - 1);
    var inter: Inter;
    inter.position = vec4<f32>(clipped_space, 0, 1);
    return inter;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return uni.color;
}
