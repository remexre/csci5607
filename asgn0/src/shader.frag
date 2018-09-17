#version 150 core

uniform sampler2D tex0;

in vec3 link_color;
in vec2 link_uv;

void main() {
	vec3 color = vec3(texture(tex0, link_uv).rgb);
	color += link_color;

	gl_FragColor = vec4(color / 2, 1.0);
}
