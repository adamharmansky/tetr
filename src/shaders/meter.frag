#version 330 core
out vec4 final_color;

in vec2 frag_texcoords;
in vec3 frag_normals;

uniform float opacity;

void main() {
    final_color = vec4(1.0, 0.5, 0.0, opacity);
}
