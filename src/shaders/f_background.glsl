#version 150 core
in vec2 f_tex_coord;

out vec4 out_color;

uniform sampler2D t_sprites;
uniform mat4 u_transform;
uniform float u_frames_count;
uniform vec2 u_offset;
uniform vec2 u_dimensions;

void main() {
   float prev_y = f_tex_coord.y;
   vec4 st = u_transform * vec4(f_tex_coord, 0.0, 1.0);
   float y_delta = st.y - prev_y;
   float speedup[5] = float[](0.0, 0.3, 0.6, -20.0, -30.0);

   for(int i=0; i<u_frames_count; ++i) {
      st.y -= speedup[i] * y_delta;
      float y = st.y > 0.0 ? fract(st.y) : 1.0 - fract(-st.y);
      float x = st.x > 0.0 ? fract(st.x) : 1.0 - fract(-st.x);
      vec2 tex_coord = vec2(u_offset.x + i*u_dimensions.x, u_offset.y)
         + vec2(x, y) * u_dimensions;
      out_color = texture(t_sprites, tex_coord);
      if (out_color.w == 1.0) {
         break;
      }
   }
}
