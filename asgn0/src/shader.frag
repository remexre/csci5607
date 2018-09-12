#version 150 core

uniform sampler2D tex0;

in vec3 link_color;
in vec2 link_uv;

out vec3 color;

void main() {
	color = texture(tex0, link_uv).rgb;
	// TODO: Blend in link_color
}
