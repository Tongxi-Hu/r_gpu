
struct Input {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct Inter {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(in: Input)-> Inter {
    var inter: Inter;
    inter.pos = vec4<f32>( in.position, 1.0);
    inter.color= in.color;
    return inter;
}

@fragment
fn fs_main(inter: Inter)-> @location(0) vec4<f32> {
    return vec4<f32>(inter.color, 1.0);
}
