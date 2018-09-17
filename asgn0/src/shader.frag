#version 150 core

uniform float brightness;
uniform bool color;
uniform sampler2D tex0;

in vec3 link_color;
in vec2 link_uv;

void main() {
	vec3 colorVec = vec3(texture(tex0, link_uv).rgb);
	int colorComponents = 1;

	if(color) {
		colorComponents++;
		colorVec += link_color;
	}

	colorVec *= brightness;
	gl_FragColor = vec4(colorVec / colorComponents, 1.0);
}
