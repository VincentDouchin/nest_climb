#import bevy_pbr::mesh_vertex_output MeshVertexOutput
struct CustomMaterial {
    margins: vec4<f32>,
    size: vec2<f32>,
    scale:vec2<f32>
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

fn map( value:f32, originalMin:f32, originalMax:f32, newMin:f32, newMax:f32) -> f32 {
    return (value - originalMin) / (originalMax - originalMin) * (newMax - newMin) + newMin;
} 

fn process_axis( coord:f32, pixel:f32, texture_pixel:f32 , start:f32, end:f32) -> f32 {
	if (coord > 1.0 - end * pixel) {
		return map(coord, 1.0 - end * pixel, 1.0, 1.0 - texture_pixel * end, 1.0);
	} else if (coord > start * pixel) {
		return map(coord, start * pixel, 1.0 - end * pixel, start * texture_pixel, 1.0 - end * texture_pixel);
	} else {
		return map(coord, 0.0, start * pixel, 0.0, start * texture_pixel);
	}
}

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
  	var pixel_size = (1.0 / material.size) / material.scale;

	var mappedUV = vec2(
		process_axis(mesh.uv.x, pixel_size.x, 1.0/material.size.x, material.margins.x, material.margins.y),
		process_axis(mesh.uv.y, pixel_size.y, 1.0/material.size.y, material.margins.z,material.margins.w)
	);
	return textureSample(base_color_texture,base_color_sampler, mappedUV);
}
