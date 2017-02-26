use glium;

const POSTPROCESSOR_VERTEX_SHADER: &'static str = include_str!("../shaders/v_post.glsl");

const POSTPROCESSOR_FRAGMENT_SHADER: &'static str = include_str!("../shaders/f_post.glsl");

#[derive(Copy, Clone)]
struct PostprocessorVertex {
    v_pos: [f32; 2],
    v_tex_coord: [f32; 2],
}

implement_vertex!(PostprocessorVertex, v_pos, v_tex_coord);

const POSTPROCESSOR_VERTICES: [PostprocessorVertex; 4] = [PostprocessorVertex {
                                                              v_pos: [-1.0, 1.0],
                                                              v_tex_coord: [0.0, 1.0],
                                                          },
                                                          PostprocessorVertex {
                                                              v_pos: [1.0, 1.0],
                                                              v_tex_coord: [1.0, 1.0],
                                                          },
                                                          PostprocessorVertex {
                                                              v_pos: [1.0, -1.0],
                                                              v_tex_coord: [1.0, 0.0],
                                                          },
                                                          PostprocessorVertex {
                                                              v_pos: [-1.0, -1.0],
                                                              v_tex_coord: [0.0, 0.0],
                                                          }];

const POSTPROCESSOR_INDICES: [u16; 6] = [0, 1, 2, 0, 3, 2];

pub struct PostProcessor {
    program: glium::Program,
    texture: glium::texture::Texture2d,
    shape: glium::VertexBuffer<PostprocessorVertex>,
    indices: glium::IndexBuffer<u16>,
}

impl PostProcessor {
    pub fn new<F>(facade: &F, dimentions: (u32, u32)) -> PostProcessor
        where F: glium::backend::Facade
    {
        PostProcessor {
            program: glium::Program::from_source(facade,
                                                 POSTPROCESSOR_VERTEX_SHADER,
                                                 POSTPROCESSOR_FRAGMENT_SHADER,
                                                 None)
                .expect("Can't initialize program"),
            texture: glium::texture::Texture2d::empty(facade, dimentions.0, dimentions.1)
                .expect("Can't create empty texture"),
            shape: glium::vertex::VertexBuffer::new(facade, &POSTPROCESSOR_VERTICES)
                .expect("Can't initialize backgroudn vertex buffer"),
            indices: glium::index::IndexBuffer::new(facade,
                                                    glium::index::PrimitiveType::TrianglesList,
                                                    &POSTPROCESSOR_INDICES)
                .expect("Can't build index buffer"),
        }
    }

    // pub fn draw<F, S, R>(&mut self, f: F) -> R
    //     where F: FnOnce(&mut S) -> R,
    //           S: glium::Surface
    // {
    //     f(&mut self.texture.as_surface())
    // }

    pub fn as_surface<'a>(&'a self) -> glium::framebuffer::SimpleFrameBuffer<'a> {
        self.texture.as_surface()
    }

    pub fn render<S>(&self, surface: &mut S)
        where S: glium::Surface
    {
        let uniforms = uniform! {
            t_post: self.texture.sampled().anisotropy(1)
                .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest),
        };
        surface.draw(&self.shape,
                  &self.indices,
                  &self.program,
                  &uniforms,
                  &Default::default())
            .expect("Can't draw");
    }
}
