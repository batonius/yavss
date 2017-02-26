#version 150 core
in vec2 f_tex_coord;

out vec4 out_color;

uniform sampler2D t_post;

void main() {
    out_color = texture(t_post, f_tex_coord);
}
