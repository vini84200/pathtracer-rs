struct VertexInput {
    @location(0) vert_pos: vec3<f32>,
};


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput
) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(model.vert_pos, 1.0);
    output.vert_pos = output.clip_position.xyz;
    return output;
}

fn ray()

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}