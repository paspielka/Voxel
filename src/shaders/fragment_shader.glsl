#version 150

out vec4 color;
in vec3 pos;

void main() {
    color = vec4(abs(pos[0]) * 0.1, abs(pos[1]) * 0.1, abs(pos[2])* 0.1, 1.0);
}