#version 450 core
layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

in mat4 g_transform[];
flat in int g_sprite[];
flat in int g_frame[];

out vec2 f_tex_pos;
flat out int f_sprite;
flat out int f_frame;

uniform SpritesSizes {
    vec2 u_sprites_sizes[1024];
};

void main() {
    float half_width = u_sprites_sizes[g_sprite[0]].x;
    float half_height = u_sprites_sizes[g_sprite[0]].y;
    f_tex_pos = vec2(0.0, 1.0);
    f_sprite = g_sprite[0];
    f_frame = g_frame[0];
    gl_Position = gl_in[0].gl_Position + vec4(-half_width, half_height, 0.0, 0.0) * g_transform[0];
    EmitVertex();
    f_sprite = g_sprite[0];
    f_frame = g_frame[0];
    f_tex_pos = vec2(0.0, 0.0);
    gl_Position = gl_in[0].gl_Position + vec4(-half_width, -half_height, 0.0, 0.0) * g_transform[0];
    EmitVertex();
    f_sprite = g_sprite[0];
    f_frame = g_frame[0];
    f_tex_pos = vec2(1.0, 1.0);
    gl_Position = gl_in[0].gl_Position + vec4(half_width, half_height, 0.0, 0.0) * g_transform[0];
    EmitVertex();
    f_sprite = g_sprite[0];
    f_frame = g_frame[0];
    f_tex_pos = vec2(1.0, 0.0);
    gl_Position = gl_in[0].gl_Position + vec4(half_width, -half_height, 0.0, 0.0) * g_transform[0];
    EmitVertex();
    EndPrimitive();
}
