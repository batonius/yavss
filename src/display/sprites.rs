use glium;
use scene::Scene;
use sprites::SpriteObject;

const MAX_SPRITES_COUNT: usize = 1024;

#[derive(Copy, Clone)]
struct SpriteVertex {
    v_pos: [f32; 2],
    v_transform: [[f32; 4]; 4],
    v_sprite: u32,
    v_frame: u32,
    v_scale: [f32; 2],
}

implement_vertex!(SpriteVertex, v_pos, v_transform, v_sprite, v_frame, v_scale);

impl SpriteVertex {
    fn from_scene_object(scene_object: &::scene::SceneObject) -> SpriteVertex {
        use scene::{PlayerState, ObjectType};
        use cgmath;
        let (sprite, frame) = match scene_object.object_type {
            ObjectType::Player(PlayerState::Normal) => (SpriteObject::Player as u32, 0),
            ObjectType::Player(PlayerState::TiltedLeft) => (SpriteObject::Player as u32, 1),
            ObjectType::Player(PlayerState::TiltedRight) => (SpriteObject::Player as u32, 2),
            ObjectType::PlayerBullet(frame) => (SpriteObject::PlayerBullet as u32, frame % 4),
            ObjectType::EnemyBullet(frame) => (SpriteObject::EnemyBullet as u32, frame % 4),
        };
        let transform = cgmath::Matrix4::from(cgmath::Quaternion::from(cgmath::Euler {
                x: cgmath::Deg(0.0),
                y: cgmath::Deg(0.0),
                z: cgmath::Deg(scene_object.sprite_angle().as_deg()),
            }))
            .into();
        SpriteVertex {
            v_pos: [scene_object.pos.x(), scene_object.pos.y()],
            v_sprite: sprite,
            v_frame: frame,
            v_transform: transform,
            v_scale: scene_object.sprite_scale().into(),
        }
    }
}

const SPRITE_VERTEX_SHADER: &'static str = include_str!("../shaders/v_sprites.glsl");

const SPRITE_FRAGMENT_SHADER: &'static str = include_str!("../shaders/f_sprites.glsl");

const SPRITE_GEOMETRY_SHADER: &'static str = include_str!("../shaders/g_sprites.glsl");

pub struct Sprites {
    sizes_ub: glium::uniforms::UniformBuffer<[[f32; 2]; MAX_SPRITES_COUNT]>,
    offsets_ub: glium::uniforms::UniformBuffer<[[f32; 2]; MAX_SPRITES_COUNT]>,
    dimensions_ub: glium::uniforms::UniformBuffer<[[f32; 2]; MAX_SPRITES_COUNT]>,
    program: glium::Program,
}

impl Sprites {
    pub fn new<F>(facade: &F, sprites_data: &::sprites::SpritesData) -> Sprites
        where F: glium::backend::Facade
    {
        let program = glium::program::Program::from_source(facade,
                                                           SPRITE_VERTEX_SHADER,
                                                           SPRITE_FRAGMENT_SHADER,
                                                           Some(SPRITE_GEOMETRY_SHADER))
                .expect("Can't compile sprites program");
        let mut sizes_ub =
            glium::uniforms::UniformBuffer::<[[f32; 2]; 1024]>::empty_immutable(facade)
                .expect("Can't create uniform buffer");
        let mut offsets_ub =
            glium::uniforms::UniformBuffer::<[[f32; 2]; 1024]>::empty_immutable(facade)
                .expect("Can't create uniform buffer");
        let mut dimensions_ub =
            glium::uniforms::UniformBuffer::<[[f32; 2]; 1024]>::empty_immutable(facade)
                .expect("Can't create uniform buffer");
        {
            let mut sizes_map = sizes_ub.map();
            let mut offsets_map = offsets_ub.map();
            let mut dimensions_map = dimensions_ub.map();
            for sprite_object in sprites_data.sprite_objects() {
                let sprite_data = sprites_data
                    .sprite_data(*sprite_object)
                    .expect("Can't get sprite data");
                sizes_map[*sprite_object as usize] = sprite_data.virtual_size().into();
                offsets_map[*sprite_object as usize] = sprite_data.image_offset().into();
                dimensions_map[*sprite_object as usize] = sprite_data.image_size().into();
            }
        }
        Sprites {
            program: program,
            sizes_ub: sizes_ub,
            offsets_ub: offsets_ub,
            dimensions_ub: dimensions_ub,
        }
    }

    pub fn render<S, F>(&self,
                        facade: &F,
                        surface: &mut S,
                        sprites_texture: &glium::texture::SrgbTexture2d,
                        scene: &Scene)
        where S: glium::Surface,
              F: glium::backend::Facade
    {
        let vertices: Vec<SpriteVertex> = scene
            .objects()
            .map(SpriteVertex::from_scene_object)
            .collect();
        let vertex_buffer = glium::vertex::VertexBuffer::new(facade, &vertices)
            .expect("Can't initialize vertex buffer");
        surface
            .draw(&vertex_buffer,
                  glium::index::NoIndices(glium::index::PrimitiveType::Points),
                  &self.program,
                  &uniform!{SpritesSizes: &self.sizes_ub,
                            SpritesOffsets: &self.offsets_ub,
                            SpritesDimensions: &self.dimensions_ub,
                            t_sprites: sprites_texture.sampled().anisotropy(1)
                            .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat)
                            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                            .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)},
                  &Default::default())
            .expect("Can't draw sprites");
    }
}
