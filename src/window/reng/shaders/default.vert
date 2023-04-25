#version 450

layout(set=0, binding=0, std140)
uniform Uniforms{
	mat4 ortho;
};

struct Instance {
	vec4  tint;
	vec4  text_coords;
	vec2  scale;
	vec2  translate;
	float rotation;
	int   screen_relative;
};

layout(set=1, binding=0, std140)
buffer InstanceData {
	Instance instances[];
};

layout(location=0) out vec2 text_coords;
layout(location=1) out vec4 color_tint;

vec2 positions[4] = vec2[](
    vec2(1.0, -1.0),
    vec2(-1.0, 1.0),
    vec2(1.0, 1.0),
    vec2(-1.0, -1.0)
);

Instance inst = instances[gl_InstanceIndex];

vec2 inst_coords[4] = vec2[](
	inst.text_coords.zw,
	inst.text_coords.xy,
	inst.text_coords.zy,
	inst.text_coords.xw
);

vec2 rotv2(vec2 vec, float theta) {
	float a = degrees(atan(vec.y, vec.x));

	return length(vec) * vec2(
		cos(radians(a + theta)),
		sin(radians(a + theta))
	);
}

void main() {

	int index = gl_VertexIndex % 4;

	vec2 coord = positions[index] / 2.;

    vec2 pos = rotv2(coord, inst.rotation) * inst.scale + inst.translate;
		
	text_coords = inst_coords[index];
	color_tint  = inst.tint;

	gl_Position = vec4(pos.x, pos.y, 0.0, 1.0);
	if (inst.screen_relative == 0) {
		gl_Position = ortho * gl_Position;
	}
}