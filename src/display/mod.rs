use glium;
use scene;

mod background;
mod sprites;
mod postprocessor;

pub struct Renderer {
    background: background::Background,
    sprites: sprites::Sprites,
    sprites_texture: glium::texture::SrgbTexture2d,
    postprocessor: postprocessor::PostProcessor,
}

impl Renderer {
    pub fn new<F>(facade: &F,
                  sprites_data: &::sprites_data::SpritesData,
                  virtual_dimensions: (u32, u32))
                  -> Renderer
        where F: glium::backend::Facade
    {
        let glium_image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(sprites_data.get_image_buffer(),
                                                               sprites_data.get_image_size());
        let texture = glium::texture::SrgbTexture2d::new(facade, glium_image)
            .expect("Can't create texture");
        Renderer {
            background: background::Background::new(facade, sprites_data),
            sprites: sprites::Sprites::new(facade, sprites_data),
            sprites_texture: texture,
            postprocessor: postprocessor::PostProcessor::new(facade, virtual_dimensions),
        }
    }

    pub fn render(&mut self,
                  window: &glium::backend::glutin_backend::GlutinFacade,
                  scene: &scene::Scene) {
        use glium::Surface;
        let mut surface = window.draw();
        self.postprocessor.draw(|framebuffer| {
            framebuffer.clear_color(0.5, 0.5, 0.0, 1.0);
            self.background.render(framebuffer,
                                   &self.sprites_texture,
                                   scene.background_position());
            self.sprites.render(window,
                                framebuffer,
                                &self.sprites_texture,
                                &scene.get_scene_objects());
        });
        self.postprocessor.render(&mut surface);
        surface.finish().expect("Can't draw on a surface");
    }
}
