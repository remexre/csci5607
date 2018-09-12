#version 150 core

uniform mat4 matrix;

in vec2 pos;
in vec3 color;
in vec2 uv;

out vec3 link_color;
out vec2 link_uv;

void main() {
	link_color = color;
	link_uv = uv;

	gl_Position = vec4(pos, 0.0, 1.0) * matrix;
}
