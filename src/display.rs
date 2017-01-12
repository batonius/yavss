use glium;
use scene;
use image;
use std;

const BACKGROUND_TILE_WIDHT: usize = 32;
const BACKGROUND_TILE_HEIGHT: usize = 32;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

const WIDTH_TILES_COUNT: f32 = (WINDOW_WIDTH as f32) / (BACKGROUND_TILE_WIDHT as f32) / 6.0;
const HEIGHT_TILES_COUNT: f32 = (WINDOW_HEIGHT as f32) / (BACKGROUND_TILE_HEIGHT as f32) / 6.0;

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

void main() {
   vec4 new_st = u_transform * vec4(out_st, 0.0, 1.0);
   out_color = texture(t_background, new_st.st);
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
                                                        in_st: [WIDTH_TILES_COUNT, 0.0],
                                                    },
                                                    BackgroundVertex {
                                                        in_pos: [1.0, -1.0],
                                                        in_st: [WIDTH_TILES_COUNT,
                                                                HEIGHT_TILES_COUNT],
                                                    },
                                                    BackgroundVertex {
                                                        in_pos: [-1.0, -1.0],
                                                        in_st: [0.0, HEIGHT_TILES_COUNT],
                                                    }];

const BACKGROUND_INDICES: [u16; 6] = [0, 1, 2, 0, 3, 2];

fn load_png_texture<F>(facade: &F, data: &[u8]) -> glium::texture::SrgbTexture2d
    where F: glium::backend::Facade
{
    let image = image::load(std::io::Cursor::new(data), image::PNG)
        .expect("Can't read png texture")
        .to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(),
                                                                   image_dimensions);
    glium::texture::SrgbTexture2d::new(facade, image).expect("Can't create texture")
}

pub struct Renderer {
    background_texture: glium::texture::SrgbTexture2d,
    background_shape: glium::VertexBuffer<BackgroundVertex>,
    background_indices: glium::IndexBuffer<u16>,
    background_program: glium::Program,
}

impl Renderer {
    pub fn new<F>(facade: &F) -> Renderer
        where F: glium::backend::Facade
    {
        Renderer {
            background_texture: load_png_texture(facade, include_bytes!("../data/background.png")),
            background_shape: glium::vertex::VertexBuffer::new(facade, &BACKGROUND_VERTICES)
                .expect("Can't initialize backgroudn vertex buffer"),
            background_indices:
                glium::index::IndexBuffer::new(facade,
                                               glium::index::PrimitiveType::TrianglesList,
                                               &BACKGROUND_INDICES)
                .expect("Can't build index buffer"),
            background_program: glium::Program::from_source(facade,
                                                            BACKGROUND_VERTEX_SHADER,
                                                            BACKGROUND_FRAGMENT_SHADER,
                                                            None)
                .expect("Can't initialize program"),
        }
    }

    pub fn render<S>(&self, surface: &mut S, scene: &scene::Scene)
        where S: glium::Surface
    {
        use cgmath;

        let u_translation: [[f32; 4]; 4] =
            cgmath::Matrix4::from_translation(cgmath::vec3(0.0, -scene.background_position(), 0.0))
                .into();
        let uniforms = uniform! {
            u_transform: u_translation,
            t_background: self.background_texture.sampled().anisotropy(1)
                .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
        };
        surface.clear_color(0.5, 0.5, 0.0, 1.0);
        surface.draw(&self.background_shape,
                  &self.background_indices,
                  &self.background_program,
                  &uniforms,
                  &Default::default())
            .expect("Can't draw");
    }
}
