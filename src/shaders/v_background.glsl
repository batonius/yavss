#version 150 core
in vec2 v_pos;
in vec2 v_tex_coord;

out vec2 f_tex_coord;

void main() {
    f_tex_coord = v_tex_coord;
    gl_Position = vec4(v_pos, 0.0, 1.0);
}