use glium;

const BACKGROUND_TILES_COUNT: usize = 3;

const BACKGROUND_VERTEX_SHADER: &'static str = include_str!("../shaders/v_background.glsl");

const BACKGROUND_FRAGMENT_SHADER: &'static str = include_str!("../shaders/f_background.glsl");

#[derive(Copy, Clone)]
struct BackgroundVertex {
    v_pos: [f32; 2],
    v_tex_coord: [f32; 2],
}

implement_vertex!(BackgroundVertex, v_pos, v_tex_coord);

const BACKGROUND_VERTICES: [BackgroundVertex; 4] =
    [BackgroundVertex {
         v_pos: [-1.0, 1.0],
         v_tex_coord: [0.0, 0.0],
     },
     BackgroundVertex {
         v_pos: [1.0, 1.0],
         v_tex_coord: [BACKGROUND_TILES_COUNT as f32, 0.0],
     },
     BackgroundVertex {
         v_pos: [1.0, -1.0],
         v_tex_coord: [BACKGROUND_TILES_COUNT as f32, BACKGROUND_TILES_COUNT as f32],
     },
     BackgroundVertex {
         v_pos: [-1.0, -1.0],
         v_tex_coord: [0.0, BACKGROUND_TILES_COUNT as f32],
     }];

const BACKGROUND_INDICES: [u16; 6] = [0, 1, 2, 0, 3, 2];

pub struct Background {
    shape: glium::VertexBuffer<BackgroundVertex>,
    indices: glium::IndexBuffer<u16>,
    program: glium::Program,
    sprite_offset: [f32; 2],
    sprite_dimensions: [f32; 2],
    frames_count: u32,
}

impl Background {
    pub fn new<F>(facade: &F, sprites_data: &::sprites_data::SpritesData) -> Background
        where F: glium::backend::Facade
    {
        let background_sprite_data =
            sprites_data.get_sprite_data(::sprites_data::SpriteObject::Background)
                .expect("Can't get background sprite");
        Background {
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
            sprite_offset: background_sprite_data.get_image_offset(),
            sprite_dimensions: background_sprite_data.get_image_size(),
            frames_count: background_sprite_data.get_frames_count(),
        }
    }

    pub fn render<S>(&self,
                     surface: &mut S,
                     sprites_texture: &glium::texture::SrgbTexture2d,
                     position: f32)
        where S: glium::Surface
    {
        use cgmath;

        let u_translation: [[f32; 4]; 4] =
            cgmath::Matrix4::from_translation(cgmath::vec3(0.0, -position, 0.0)).into();
        let uniforms = uniform! {
            u_transform: u_translation,
            t_sprites: sprites_texture.sampled().anisotropy(1)
                .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest),
            u_frames_count: self.frames_count as f32,
            u_offset: self.sprite_offset,
            u_dimensions: self.sprite_dimensions,
        };
        surface.draw(&self.shape,
                  &self.indices,
                  &self.program,
                  &uniforms,
                  &Default::default())
            .expect("Can't draw");
    }
}
