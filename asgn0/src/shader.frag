#version 150 core

uniform sampler2D tex0;

in vec3 link_color;
in vec2 link_uv;

void main() {
	gl_FragColor = vec4(texture(tex0, link_uv).rgb, 1.0);
	// TODO: Blend in link_color
}
