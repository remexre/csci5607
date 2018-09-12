#version 150 core

in vec2 pos;
in vec2 uv;

out vec2 tex_coord;

void main() {
   tex_coord = uv;

   gl_Position = vec4(pos, 0.0, 1.0);
}
