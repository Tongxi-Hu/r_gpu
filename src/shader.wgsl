
struct VertexInput{
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct Inter{
    @builtin(position) pos:vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(vertex_in: VertexInput)->Inter{
    var out: Inter;
    out.pos = vec4<f32>(vertex_in.position, 1.0);
    out.color=vertex_in.color;
    return out;
}

@fragment
fn fs_main(data: Inter)-> @location(0) vec4<f32>{
    return vec4<f32>(data.color, 1.0);
}
