use glium;
use scene;

mod common;
mod background;

pub struct Renderer {
    background: background::Background,
}

impl Renderer {
    pub fn new<F>(facade: &F) -> Renderer
        where F: glium::backend::Facade
    {
        Renderer {
            background: background::Background::new(facade),
        }
    }

    pub fn render<S>(&self, surface: &mut S, scene: &scene::Scene)
        where S: glium::Surface
    {
        surface.clear_color(0.5, 0.5, 0.0, 1.0);
        self.background.render(surface, scene.background_position());
    }
}
