#version 330 core
out vec4 final_color;

in vec2 frag_texcoords;
in vec3 frag_normals;

uniform vec4 kolor;

void main() {
    final_color = kolor;
}
