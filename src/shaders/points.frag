#version 140
out vec4 color;

const vec3 points_color = vec3(1.0, 1.0, 1.0);

void main() {
    color = vec4(points_color, 1.0);
}
