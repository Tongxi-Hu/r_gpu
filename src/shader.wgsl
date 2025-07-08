@vertex
fn vs_main(@builtin(vertex_index) v_in:u32, @builtin(instance_index) instance_in:u32)->@builtin(position) vec4<f32>{
    let x = f32(i32(v_in)-1)* 0.5 + f32(instance_in)*0.5;
    let y = f32(i32(v_in & 1u)*2-1) * 0.5 + f32(instance_in)*0.5;
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main()-> @location(0) vec4<f32>{
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
