#version 330 core
out vec4 final_color;

in vec2 frag_texcoords;
in vec3 frag_normals;

uniform sampler2D textur;
uniform vec4 color;

void main() {
    vec4 vsa = texture(textur, frag_texcoords);
    final_color = vec4(color.xyz, 0.0) * vsa.y + vec4(1.0, 1.0, 1.0, 0.0) * (vsa.x - vsa.y);
    final_color.w = vsa.z * color.w;
}
