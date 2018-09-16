#version 150 core

uniform float aspect_ratio;
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

	mat2 rot = mat2(cos(rotation), -sin(rotation), sin(rotation), cos(rotation));
	vec2 xy = pos * rot * scale + off;
	gl_Position = vec4(xy.x, xy.y * aspect_ratio, 0.0, 1.0);
}
