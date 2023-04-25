#version 450
layout(location=0) in vec2 text_coords;
layout(location=1) flat in vec4 color_tint;

layout(location=0) out vec4 out_color;

layout(set=0, binding=0, std140)
uniform Uniforms{
	mat4 ortho;
};

layout(set = 2, binding = 0) uniform texture2D text;
layout(set = 2, binding = 1) uniform sampler samp;

void main() {
	out_color = texture(sampler2D(text, samp), text_coords) * pow(color_tint, vec4(2.2));
	if (out_color.a == 0.0) {
		discard;
	}
}