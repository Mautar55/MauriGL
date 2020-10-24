#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;

uniform mat4 p_matrix;
uniform mat4 v_matrix;
uniform mat4 t_matrix;

void main() {
    v_normal = transpose(inverse(mat3(t_matrix))) * normal;
    gl_Position = p_matrix * v_matrix * t_matrix * vec4(position, 1.0);
}