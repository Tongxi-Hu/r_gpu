struct Scene {
    view: vec4<f32>,
    light_position: vec4<f32>,
    light_direction: vec4<f32>,
    eye_position: vec4<f32>,
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

struct Inter {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) surface_vector: vec4<f32>,
    @location(2) surface_light_vector: vec4<f32>,
    @location(3) surface_eye_vector: vec4<f32>,
    @location(4) light_direction: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> scene: Scene;

@group(1) @binding(0)
var<uniform> tran: Transform;

const PI: f32 = 3.141592653589793238462643;

const SHININESS: f32= 512.0;

@vertex
fn vs_main(in: Input) -> Inter {
    // object space transformation
    let transformed = in.position * (tran.scale * tran.rotation * tran.translation);

    var inter: Inter;
    inter.position = transformed * to_clip_space(scene.view);
    inter.color = in.color;
    inter.surface_vector = in.norm * tran.rotation;
    inter.surface_light_vector = scene.light_position - transformed;
    inter.surface_eye_vector = scene.eye_position - transformed;
    inter.light_direction = scene.light_direction;
    return inter;
}

@fragment
fn fs_main(inter: Inter) -> @location(0) vec4<f32> {
    return lighting(inter.color, inter.surface_vector, inter.surface_light_vector, inter.surface_eye_vector, inter.light_direction);
}

// (width, height, near, far)
fn to_clip_space(view: vec4<f32>) -> mat4x4<f32> {
    return mat4x4<f32>(//
        vec4<f32>(2.0 * (- view[2]) / view[0], 0.0, 0.0, 0.0), // x
        vec4<f32>(0.0, 2.0 * (- view[2]) / view[1], 0.0, 0.0), // y
        vec4<f32>(0.0, 0.0, (- view[3]) / (view[3] - view[2]), view[2] * view[3] / (view[3] - view[2])), // z
        vec4<f32>(0.0, 0.0, - 1.0, 0.0));
    // w
}

fn lighting(color: vec4<f32>, surface_vector: vec4<f32>, surface_light_vector: vec4<f32>, surface_eye_vector: vec4<f32>, light_direction: vec4<f32>) -> vec4<f32> {
    let surface_light_norm = normalize(surface_light_vector.xyz);
    let light_direction_norm = - normalize(light_direction.xyz);
    let align = dot(surface_light_norm, light_direction_norm);
    if align > 0.5 {
        let surface_norm = normalize(surface_vector.xyz);
        let surface_eye_norm = normalize(surface_eye_vector.xyz);
        let half_norm = normalize(surface_eye_norm + surface_light_norm);
        let light = dot(surface_norm, surface_light_norm);
        var specular = dot(surface_norm, half_norm);
        specular = select(0.0, pow(specular, SHININESS), specular > 0.0);
        let color_with_light = vec4<f32>(color.xyz * light + specular, color.w);
        return color_with_light;
    } else {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
}