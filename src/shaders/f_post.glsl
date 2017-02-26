#version 150 core
in vec2 f_tex_coord;

out vec4 out_color;

uniform sampler2D t_post;
uniform uint u_virtual_width;
uniform uint u_virtual_height;

void main() {
    out_color = texture(t_post, f_tex_coord);
    if (fract(f_tex_coord.x * float(u_virtual_width)) < 0.2
        || fract(f_tex_coord.y * float(u_virtual_height)) < 0.2) {
        out_color *= vec4(0.5, 0.5, 0.5, 1.0);
    }
}
