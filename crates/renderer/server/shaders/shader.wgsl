
struct Transform{
    matrix: mat4x4<f32>,
}

struct VertexOutput{
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
}

struct VertexInput{
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
}

@group(1) @binding(0)
var<uniform> transform: Transform;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    output.position = transform.matrix * vec4<f32>(input.position, 1.0);
    output.tex_coord = input.tex_coord;

    return output;
}


@group(0) @binding(0)
var t_texture: texture_2d<f32>;
@group(0) @binding(1)
var t_sampler: sampler;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_texture, t_sampler, input.tex_coord);
}