#version 150 core

uniform vec2  off;
uniform float rotation;
uniform float scale;

in vec2 pos;
in vec3 color;
in vec2 uv;

out vec3 link_color;
out vec2 link_uv;

void main() {
	link_color = color;
	link_uv = uv;

	vec2 xy = pos * scale + off;
	mat2 rot = mat2(cos(rotation), -sin(rotation), sin(rotation), cos(rotation));
	gl_Position = vec4(xy * rot, 0.0, 1.0);
}
