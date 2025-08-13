struct Uniform {
    view: vec4<f32>,
    light_position: vec4<f32>,
    eye_position:vec4<f32>,
    rotation: vec4<f32>,
    translation: vec4<f32>,
}

struct Input {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) norm: vec3<f32>,
}

struct Inter {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) surface_vector: vec4<f32>,
    @location(2) surface_light_vector: vec4<f32>,
    @location(3) surface_eye_vector: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> uni: Uniform;

const PI: f32 = 3.141592653589793238462643;

const SHININESS: f32= 2500.0;

@vertex
fn vs_main(in: Input) -> Inter {
    // physical space transformation 3*3 mat
    let rotation_x = rotation_3d_x(uni.rotation.x);
    let rotation_y = rotation_3d_y(uni.rotation.y);
    let rotation_z = rotation_3d_z(uni.rotation.z);
    let translation = translation_3d(uni.translation);
    let transformed = vec4<f32>(in.position, 1) * (rotation_x * rotation_y * rotation_z * translation);


    var inter: Inter;
    inter.position = transformed * to_clip_space(uni.view);
    inter.color = vec4<f32>(in.color,1);
    inter.surface_vector = vec4<f32>(in.norm, 1) * (rotation_x * rotation_y * rotation_z);
    inter.surface_light_vector = uni.light_position - transformed;
    inter.surface_eye_vector =  uni.eye_position - transformed;
    return inter;
}

@fragment
fn fs_main(inter: Inter) -> @location(0) vec4<f32> {
    //let color = lighting(inter.color, inter.surface_vector, inter.surface_light_vector, inter.surface_eye_vector);
    return  inter.color;
}

// generate 2d scaling matrix
fn scaling_2d(scaling: f32) -> mat3x3<f32> {
    return mat3x3<f32>(vec3<f32>(scaling, 0, 0), vec3<f32>(0, scaling, 0), vec3<f32>(0, 0, 1));
}

// generate 2d rotation matrix
fn rotation_2d(rotation_deg: f32) -> mat3x3<f32> {
    return mat3x3<f32>(vec3<f32>(cos(PI * rotation_deg / 180.0), - sin(PI * rotation_deg / 180.0), 0), vec3<f32>(sin(PI * rotation_deg / 180.0), cos(PI * rotation_deg / 180.0), 0), vec3<f32>(0, 0, 1));
}

// generate 2d translation matrix
fn translation_2d(translation: vec2<f32>) -> mat3x3<f32> {
    return mat3x3<f32>(vec3<f32>(1, 0, translation.x), vec3<f32>(0, 1, translation.y), vec3<f32>(0, 0, 1));
}

// generate 3d scaling matrix
fn scaling_3d(scaling: vec4<f32>) -> mat4x4<f32> {
    return mat4x4<f32>(vec4<f32>(scaling.x, 0, 0, 0), vec4<f32>(0, scaling.y, 0, 0), vec4<f32>(0, 0, scaling.z, 0), vec4<f32>(0, 0, 0, 1));
}

// generate 3d rotation matrix around x axis
fn rotation_3d_x(rotation_deg: f32) -> mat4x4<f32> {
    return mat4x4<f32>(vec4<f32>(1, 0, 0, 0), vec4<f32>(0, cos(PI * rotation_deg / 180.0), - sin(PI * rotation_deg / 180.0), 0,), vec4<f32>(0, sin(PI * rotation_deg / 180.0), cos(PI * rotation_deg / 180.0), 0), vec4<f32>(0, 0, 0, 1));
}

// generate 3d rotation matrix around y axis
fn rotation_3d_y(rotation_deg: f32) -> mat4x4<f32> {
    return mat4x4<f32>(vec4<f32>(cos(PI * rotation_deg / 180.0), 0, sin(PI * rotation_deg / 180.0), 0), vec4<f32>(0, 1, 0, 0), vec4<f32>(- sin(PI * rotation_deg / 180.0), 0, cos(PI * rotation_deg / 180.0), 0), vec4<f32>(0, 0, 0, 1));
}

// generate 3d rotation matrix around z axis
fn rotation_3d_z(rotation_deg: f32) -> mat4x4<f32> {
    return mat4x4<f32>(vec4<f32>(cos(PI * rotation_deg / 180.0), - sin(PI * rotation_deg / 180.0), 0, 0), vec4<f32>(sin(PI * rotation_deg / 180.0), cos(PI * rotation_deg / 180.0), 0, 0), vec4<f32>(0, 0, 1, 0), vec4<f32>(0, 0, 0, 1));
}

// generate 3d translation matrix
fn translation_3d(translation: vec4<f32>) -> mat4x4<f32> {
    return mat4x4<f32>(vec4<f32>(1, 0, 0, translation.x), vec4<f32>(0, 1, 0, translation.y), vec4<f32>(0, 0, 1, translation.z), vec4<f32>(0, 0, 0, 1));
}

// (width, height, near, far)
fn to_clip_space(view: vec4<f32>) -> mat4x4<f32> {   
    return mat4x4<f32>(//
    vec4<f32>(2 * (- view[2]) / view[0], 0, 0, 0), // x
    vec4<f32>(0, 2 * (- view[2]) / view[1], 0, 0), // y
    vec4<f32>(0, 0, (- view[3]) / (view[3] - view[2]), view[2] * view[3] / (view[3] - view[2])), // z
    vec4<f32>(0, 0, - 1, 0));
    // w
}

fn lighting(color:vec4<f32>,surface_vector:vec4<f32>,surface_light_vector:vec4<f32>,surface_eye_vector:vec4<f32>)->vec4<f32>{
    let surface_light_norm = normalize(surface_light_vector.xyz);
    let surface_norm = normalize(surface_vector.xyz);
    let surface_eye_norm = normalize(surface_eye_vector.xyz);
    let half_norm = normalize(surface_eye_norm + surface_light_norm);
    let light = dot(surface_norm, surface_light_norm);
    var specular = dot(surface_norm, half_norm);
    specular = select(0.0, pow(specular, SHININESS), specular > 0.0); 
    let color_with_light = vec4<f32>(color.xyz * light + specular, color.w);
    return color_with_light;
}