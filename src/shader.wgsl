struct Uniform {
    color: vec4<f32>,
    resolution: vec3<f32>,
    scaling: f32,
    rotation: vec3<f32>,
    translation: vec3<f32>,
}

struct Input {
    @location(0) position: vec4<f32>,
}

struct Inter {
    @builtin(position) position: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> uni: Uniform;

@vertex
fn vs_main(in: Input) -> Inter {
    // physical space transformation 3*3 mat
    let scaling = scaling_3d(uni.scaling);
    let rotation_x = rotation_3d_x(uni.rotation.x);
    let rotation_y = rotation_3d_y(uni.rotation.y);
    let rotation_z = rotation_3d_z(uni.rotation.z);
    let translation = translation_3d(uni.translation);
    let transformed = in.position * (scaling * rotation_x * rotation_y * rotation_z * translation);
    // view space transformation
    let clipped_space = transformed * to_clip_space(uni.resolution);
    var inter: Inter;
    inter.position = clipped_space;
    return inter;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return uni.color;
}

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

// generate 3d scaling matrix
fn scaling_3d(scaling: f32) -> mat4x4<f32> {
    return mat4x4<f32>(vec4<f32>(scaling, 0, 0, 0), vec4<f32>(0, scaling, 0, 0), vec4<f32>(0, 0, scaling, 0), vec4<f32>(0, 0, 0, 1));
}

// generate 3d rotation matrix around x axis
fn rotation_3d_x(rotation_deg: f32) -> mat4x4<f32> {
    const PI: f32 = 3.141592653589793238462643;
    return mat4x4<f32>(vec4<f32>(1, 0, 0, 0), vec4<f32>(cos(PI * rotation_deg / 180.0), - sin(PI * rotation_deg / 180.0), 0, 0), vec4<f32>(sin(PI * rotation_deg / 180.0), cos(PI * rotation_deg / 180.0), 0, 0), vec4<f32>(0, 0, 0, 1));
}

// generate 3d rotation matrix around y axis
fn rotation_3d_y(rotation_deg: f32) -> mat4x4<f32> {
    const PI: f32 = 3.141592653589793238462643;
    return mat4x4<f32>(vec4<f32>(cos(PI * rotation_deg / 180.0), 0, sin(PI * rotation_deg / 180.0), 0), vec4<f32>(0, 1, 0, 0), vec4<f32>(- sin(PI * rotation_deg / 180.0), 0, cos(PI * rotation_deg / 180.0), 0), vec4<f32>(0, 0, 0, 1));
}

// generate 3d rotation matrix around z axis
fn rotation_3d_z(rotation_deg: f32) -> mat4x4<f32> {
    const PI: f32 = 3.141592653589793238462643;
    return mat4x4<f32>(vec4<f32>(cos(PI * rotation_deg / 180.0), - sin(PI * rotation_deg / 180.0), 0, 0), vec4<f32>(sin(PI * rotation_deg / 180.0), cos(PI * rotation_deg / 180.0), 0, 0), vec4<f32>(0, 0, 1, 0), vec4<f32>(0, 0, 0, 1));
}

// generate 3d translation matrix
fn translation_3d(translation: vec3<f32>) -> mat4x4<f32> {
    return mat4x4<f32>(vec4<f32>(1, 0, 0, uni.translation.x), vec4<f32>(0, 1, 0, uni.translation.y), vec4<f32>(0, 0, 1, uni.translation.z), vec4<f32>(0, 0, 0, 1));
}

fn to_clip_space(resolution: vec3<f32>) -> mat4x4<f32> {
    return mat4x4(vec4<f32>(2 / resolution.x, 0, 0, 0), vec4<f32>(0, - 2 / resolution.y, 0, 0), vec4<f32>(0, 0, 0.5 / resolution.z, 0), vec4<f32>(- 1, 1, 0.5, 1));
}