use glium;
use image;
use std;

pub fn load_png_texture<F>(facade: &F, data: &[u8]) -> glium::texture::SrgbTexture2d
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

