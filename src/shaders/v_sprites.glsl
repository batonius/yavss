#version 150 core
in vec2 v_pos;
in mat4 v_transform;
in int v_sprite;
in int v_frame;
in vec2 v_scale;

out mat4 g_transform;
flat out int g_sprite;
flat out int g_frame;
flat out vec2 g_scale;

void main() {
    g_transform = v_transform;
    g_sprite = v_sprite;
    g_frame = v_frame;
    g_scale = v_scale;
    gl_Position = vec4(v_pos.x * 2.0 - 1.0, (1.0 - v_pos.y) * 2.0 - 1.0, 0.0, 1.0);
}
