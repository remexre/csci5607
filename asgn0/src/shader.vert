#version 150 core

uniform mat3 matrix;

in vec2 pos;
in vec3 color;
in vec2 uv;

out vec3 link_color;
out vec2 link_uv;

void main() {
	link_color = color;
	link_uv = uv;

	mat3 matrix2 = matrix;
	//matrix2[0] = vec3(0.95, 0.05, 0);
	//matrix2[1] = vec3(0.05, 0.95, 0);
	gl_Position = vec4(vec3(pos, 0) * matrix2, 1.0);
}
