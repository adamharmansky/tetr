#version 330 core
out vec4 final_color;

in vec2 frag_texcoords;
in vec3 frag_normals;

uniform sampler2D textur;
uniform float opacity;

void main() {
    final_color = texture(textur, frag_texcoords) * vec4(1.0, 1.0, 1.0, opacity);
}
