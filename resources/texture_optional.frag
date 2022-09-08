#version 330 core
out vec4 final_color;

in vec2 frag_texcoords;
in vec3 frag_normals;

uniform sampler2D textur;
uniform vec4 color;
uniform int enable_texture;

void main() {
    if (enable_texture == 0) {
        final_color = color;
    } else {
        final_color = texture(textur, frag_texcoords) * color;
    }
}
