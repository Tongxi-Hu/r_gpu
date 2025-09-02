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

struct Inter {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) surface_vector: vec4<f32>,
    @location(2) surface_light_vector: vec4<f32>,
    @location(3) surface_eye_vector: vec4<f32>,
    @location(4) light_direction: vec4<f32>,
    @location(5) light_position: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> scene: Scene;

@group(1) @binding(0)
var<uniform> tran: Transform;

@group(2) @binding(0)
var shadow_texture: texture_depth_2d;

@group(2) @binding(1)
var shadow_sampler: sampler_comparison;

const SHININESS: f32= 512.0;

@vertex
fn vs_main(in: Input) -> Inter {
    // object space transformation
    let transformed = in.position * (tran.scale * tran.rotation * tran.translation);

    var inter: Inter;
    inter.position = transformed * scene.perspective_projection;
    inter.color = in.color;
    inter.surface_vector = in.norm * tran.rotation;
    inter.surface_light_vector = scene.light_position - transformed;
    inter.surface_eye_vector = scene.eye_position - transformed;
    inter.light_direction = scene.light_direction;
    inter.light_position = in.position * (tran.scale * tran.rotation * tran.translation) * scene.light_projection;
    return inter;
}

@fragment
fn fs_main(inter: Inter) -> @location(0) vec4<f32> {
    return lighting(inter.color, inter.surface_vector, inter.surface_light_vector, inter.surface_eye_vector, inter.light_direction, inter.light_position);
}

fn lighting(color: vec4<f32>, surface_vector: vec4<f32>, surface_light_vector: vec4<f32>, surface_eye_vector: vec4<f32>, light_direction: vec4<f32>, light_position: vec4<f32>) -> vec4<f32> {
    let surface_light_norm = normalize(surface_light_vector.xyz);
    let light_direction_norm = - normalize(light_direction.xyz);
    let align = dot(surface_light_norm, light_direction_norm);
    if align > 0.5 {
        let surface_norm = normalize(surface_vector.xyz);
        let surface_eye_norm = normalize(surface_eye_vector.xyz);
        let half_norm = normalize(surface_eye_norm + surface_light_norm);
        let diffuse = dot(surface_norm, surface_light_norm);

        var shadow: f32 = 0.0;
    // apply Percentage-closer filtering (PCF)
    // sample nearest 9 texels to smooth result
        let size = f32(textureDimensions(shadow_texture).x);
        for (var y: i32 = -1 ; y <= 1 ; y = y + 1) {
            for (var x: i32 = -1 ; x <= 1 ; x = x + 1) {
                let offset = vec2<f32>(f32(x) / size, f32(y) / size);
                shadow = shadow + textureSampleCompare(
                    shadow_texture,
                    shadow_sampler,
                    light_position.xy + offset,
                    light_position.z - 0.001  // apply a small bias to avoid acne
                );
            }
        }
        shadow = shadow / 9.0;

        var specular = dot(surface_norm, half_norm);
        specular = select(0.0, pow(specular, SHININESS), specular > 0.0);
        let color_with_light = vec4<f32>(color.xyz * diffuse * shadow + specular, color.w);
        return color_with_light;
    } else {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
}