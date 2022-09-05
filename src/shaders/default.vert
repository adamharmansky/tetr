#version 330 core
layout (location=0) in vec3 pos;
layout (location=1) in vec2 texCoords;
layout (location=2) in vec3 normals;

uniform mat4 view;

out vec2 frag_texcoords;
out vec3 frag_normals;

void main() {
    frag_texcoords = texCoords;
    frag_normals = normalize(normals);
    gl_Position = view * vec4(pos, 1.0);
}
