#version 450 core

in vec2 f_tex_pos;
flat in int f_sprite;
flat in int f_frame;

out vec4 out_color;

uniform SpritesOffsets {
    vec2 u_sprites_offsets[1024];
};

uniform SpritesDimensions {
    vec2 u_sprites_dimensions[1024];
};

uniform sampler2D t_sprites;

void main() {
    float actual_x = u_sprites_offsets[f_sprite].x +
        f_frame * u_sprites_dimensions[f_sprite].x +
        f_tex_pos.x * u_sprites_dimensions[f_sprite].x;
    float actual_y = u_sprites_offsets[f_sprite].y + f_tex_pos.y * u_sprites_dimensions[f_sprite].y;
    actual_y = 1.0 - actual_y;
    vec2 actual_tex_pos = vec2(actual_x, actual_y);
    out_color = texture(t_sprites, actual_tex_pos);
    if (out_color.w == 0.0) {
        discard;
    }
}
