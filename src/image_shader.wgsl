
struct VertexInput {
    @location(0) vert_pos: vec3<f32>,
    @location(1) vert_tex_coord: vec2<f32>,
};


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    @builtin(vertex_index) vertex_index: u32,
) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(model.vert_pos, 1.0);
    output.vert_pos = output.clip_position.xyz;
    output.tex_coord = model.vert_tex_coord;
    return output;
}


@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var s: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, s, in.tex_coord);
}