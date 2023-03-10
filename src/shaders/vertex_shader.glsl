#version 150

uniform mat4 r_matrix;
uniform mat4 p_matrix;

in vec3 position;
out vec3 pos;

void main() {
    pos = position;
    gl_Position = p_matrix * r_matrix * vec4(position, 1.0);
}
