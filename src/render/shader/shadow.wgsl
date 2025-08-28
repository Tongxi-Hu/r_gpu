struct Scene {
    perspective_projection: mat4x4<f32>,
    light_projection: mat4x4<f32>,
    light_position: vec4<f32>,
    light_direction: vec4<f32>,
    eye_position: vec4<f32>,
    eye_direction: vec4<f32>,
}

struct Transform {
    scale: mat4x4<f32>,
    rotation: mat4x4<f32>,
    translation: mat4x4<f32>,
}

struct Input {
    @location(0) position: vec4<f32>,
    @location(1) color: vec4<f32>,
    @location(2) norm: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> scene: Scene;

@group(1) @binding(0)
var<uniform> tran: Transform;

@vertex
fn shadow_main(in: Input) -> @builtin(position) vec4<f32> {
    let transformed = in.position * (tran.scale * tran.rotation * tran.translation);
    return transformed * scene.light_projection;
}