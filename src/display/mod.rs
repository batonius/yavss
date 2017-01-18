use glium;
use scene;

mod background;
mod sprites;

pub struct Renderer {
    background: background::Background,
    sprites: sprites::Sprites,
    sprites_texture: glium::texture::SrgbTexture2d,
}

impl Renderer {
    pub fn new<F>(facade: &F, sprites_data: &::sprites_data::SpritesData) -> Renderer
        where F: glium::backend::Facade
    {
        let glium_image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(sprites_data.get_image_buffer(),
                                                               sprites_data.get_dimensions());
        let texture = glium::texture::SrgbTexture2d::new(facade, glium_image)
            .expect("Can't create texture");
        Renderer {
            background: background::Background::new(facade, sprites_data),
            sprites: sprites::Sprites::new(facade, sprites_data),
            sprites_texture: texture,
        }
    }

    pub fn render(&self,
                  window: &glium::backend::glutin_backend::GlutinFacade,
                  scene: &scene::Scene) {
        use glium::Surface;
        let mut surface = window.draw();
        surface.clear_color(0.5, 0.5, 0.0, 1.0);
        self.background.render(&mut surface, &self.sprites_texture,
                               scene.background_position());
        self.sprites.render(window, &mut surface, &self.sprites_texture,
                            &scene.get_scene_objects());
        surface.finish().expect("Can't draw on a surface");
    }
}
