use glium;
use super::common::load_png_texture;

const BACKGROUND_FRAMES_COUNT: usize = 4;
const BACKGROUND_TILES_COUNT: usize = 10;

const BACKGROUND_VERTEX_SHADER: &'static str = r"
#version 150 core
in vec2 in_pos;
in vec2 in_st;

out vec2 out_st;

void main() {
    out_st = in_st;
    gl_Position = vec4(in_pos, 0.0, 1.0);
}";

const BACKGROUND_FRAGMENT_SHADER: &'static str = r"
#version 150 core
in vec2 out_st;

out vec4 out_color;

uniform sampler2D t_background;
uniform mat4 u_transform;
uniform float u_background_frames_count;


void main() {
   float y = out_st.y;
   vec4 st = u_transform * vec4(out_st, 0.0, 1.0);
   y = st.y - y;
   float speedup[5] = float[](0.0, 0.3, 0.6, -20.0, -30.0);

   for(int i=0; i<u_background_frames_count; ++i) {
      vec4 stt = vec4(st.x, st.y - speedup[i] * y, st.zw);
      float f = fract(stt.x);
      vec2 t_coord = vec2((f + i)/u_background_frames_count, stt.yzw);
      out_color = texture(t_background, t_coord);
      if (out_color.w == 1.0) {
          break;
      }
   }
}";

#[derive(Copy, Clone)]
struct BackgroundVertex {
    in_pos: [f32; 2],
    in_st: [f32; 2],
}

implement_vertex!(BackgroundVertex, in_pos, in_st);

const BACKGROUND_VERTICES: [BackgroundVertex; 4] = [BackgroundVertex {
                                                        in_pos: [-1.0, 1.0],
                                                        in_st: [0.0, 0.0],
                                                    },
                                                    BackgroundVertex {
                                                        in_pos: [1.0, 1.0],
                                                        in_st: [BACKGROUND_TILES_COUNT as f32, 0.0],
                                                    },
                                                    BackgroundVertex {
                                                        in_pos: [1.0, -1.0],
                                                        in_st: [BACKGROUND_TILES_COUNT as f32,
                                                                BACKGROUND_TILES_COUNT as f32],
                                                    },
                                                    BackgroundVertex {
                                                        in_pos: [-1.0, -1.0],
                                                        in_st: [0.0, BACKGROUND_TILES_COUNT as f32],
                                                    }];

const BACKGROUND_INDICES: [u16; 6] = [0, 1, 2, 0, 3, 2];

pub struct Background {
    texture: glium::texture::SrgbTexture2d,
    shape: glium::VertexBuffer<BackgroundVertex>,
    indices: glium::IndexBuffer<u16>,
    program: glium::Program,
}

impl Background {
    pub fn new<F>(facade: &F) -> Background
        where F: glium::backend::Facade
    {
        Background {
            texture: load_png_texture(facade, include_bytes!("../../data/background.png")),
            shape: glium::vertex::VertexBuffer::new(facade, &BACKGROUND_VERTICES)
                .expect("Can't initialize backgroudn vertex buffer"),
            indices: glium::index::IndexBuffer::new(facade,
                                                    glium::index::PrimitiveType::TrianglesList,
                                                    &BACKGROUND_INDICES)
                .expect("Can't build index buffer"),
            program: glium::Program::from_source(facade,
                                                 BACKGROUND_VERTEX_SHADER,
                                                 BACKGROUND_FRAGMENT_SHADER,
                                                 None)
                .expect("Can't initialize program"),
        }
    }

    pub fn render<S>(&self, surface: &mut S, position: f32)
        where S: glium::Surface
    {
        use cgmath;

        let u_translation: [[f32; 4]; 4] =
            cgmath::Matrix4::from_translation(cgmath::vec3(0.0, -position, 0.0)).into();
        let uniforms = uniform! {
            u_transform: u_translation,
            t_background: self.texture.sampled().anisotropy(1)
                .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest),
            u_background_frames_count: (BACKGROUND_FRAMES_COUNT as f32),
        };
        surface.draw(&self.shape,
                  &self.indices,
                  &self.program,
                  &uniforms,
                  &Default::default())
            .expect("Can't draw");
    }
}
