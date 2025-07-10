struct Uniform{
    color: vec4<f32>,
    resolution: vec2<f32>,
    scaling:vec2<f32>,
    rotation: vec2<f32>,
    translation: vec2<f32>,
}

struct Input{
    @location(0) position:vec2<f32>,
}

struct Inter{
    @builtin(position) position:vec4<f32>,
}

@group(0) @binding(0) var<uniform> uni:Uniform;

@vertex
fn vs_main(in: Input)-> Inter {
    var inter: Inter;
    // physical space transformation
    let scaled=in.position*uni.scaling;
    let rotated=vec2<f32>(
    scaled.x * uni.rotation.x - scaled.y * uni.rotation.y,
    scaled.x * uni.rotation.y + scaled.y * uni.rotation.x);
    let translated=rotated+uni.translation;
    // view space transformation
    let zero_to_one= translated/uni.resolution;
    let zero_to_two=zero_to_one * 2.0;
    let flipped= zero_to_two - 1.0;
    let clipped_space = flipped*vec2<f32>(1,-1);
    inter.position = vec4<f32>(clipped_space,0, 1);
    return inter;
}

@fragment
fn fs_main()-> @location(0) vec4<f32> {
    return uni.color;
}
