#version 150

out vec4 color;
in vec3 pos;

void main() {
    color = vec4(abs(pos) * 0.1, 1.0);
}
