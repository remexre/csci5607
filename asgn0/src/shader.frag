#version 150 core

uniform sampler2D tex0;

in vec2 tex_coord;
out vec3 color;

void main() {
   color = texture(tex0, tex_coord).rgb;
}
