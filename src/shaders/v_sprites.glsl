#version 150 core
in vec2 v_pos;
in mat4 v_transform;
in int v_sprite;
in int v_frame;

out mat4 g_transform;
flat out int g_sprite;
flat out int g_frame;

void main() {
    g_transform = v_transform;
    g_sprite = v_sprite;
    g_frame = v_frame;
    gl_Position = vec4(v_pos, 0.0, 1.0);
}
