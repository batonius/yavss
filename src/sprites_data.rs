use image;
use std::collections::HashMap;

const SPRITES_IMAGE: &'static [u8] = include_bytes!("../data/sprites.png");
const SPRITES_DESCR: &'static str = include_str!("../data/sprites.txt");

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum SpriteObject {
    Background,
    Player,
    Bullet,
}

#[derive(Debug, Copy, Clone)]
pub struct Hitbox {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[derive(Debug)]
pub struct SpriteData {
    image_offset: [f32; 2],
    image_size: [f32; 2],
    virtual_size: [f32; 2],
    hitbox: Hitbox,
    frames_count: u32,
}

impl SpriteData {
    pub fn get_image_offset(&self) -> [f32; 2] {
        self.image_offset
    }

    pub fn get_image_size(&self) -> [f32; 2] {
        self.image_size
    }

    pub fn get_virtual_size(&self) -> [f32; 2] {
        self.virtual_size
    }

    pub fn get_frames_count(&self) -> u32 {
        self.frames_count
    }

    pub fn get_hitbox(&self) -> Hitbox {
        self.hitbox
    }

    pub fn get_virtual_hitbox(&self) -> Hitbox {
        Hitbox {
            left: self.virtual_size[0] * (0.5 - self.hitbox.left),
            top: self.virtual_size[1] * (0.5 - self.hitbox.top),
            right: self.virtual_size[0] * (self.hitbox.right - 0.5),
            bottom: self.virtual_size[1] * (self.hitbox.bottom - 0.5),
        }
    }
}

#[derive(Debug)]
pub struct SpritesData {
    image_buffer: Vec<u8>,
    image_size: (u32, u32),
    sprites: HashMap<SpriteObject, SpriteData>,
}

impl SpritesData {
    pub fn from_image_buffer(image_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
                             virtual_dimensions: (u32, u32),
                             image_dimensions: (u32, u32))
                             -> SpritesData {
        let sprites =
            SpritesData::parse_sprites_descr(&image_buffer, virtual_dimensions, image_dimensions);
        SpritesData {
            image_buffer: image_buffer.into_raw(),
            image_size: image_dimensions,
            sprites: sprites,
        }
    }

    fn parse_sprite_name(name: &str) -> Option<SpriteObject> {
        match name {
            "BACKGROUND" => Some(SpriteObject::Background),
            "PLAYER" => Some(SpriteObject::Player),
            "BULLET" => Some(SpriteObject::Bullet),
            _ => None,
        }
    }

    fn parse_sprites_descr(image_buffer: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
                           virtual_dimensions: (u32, u32),
                           image_dimensions: (u32, u32))
                           -> HashMap<SpriteObject, SpriteData> {
        use std::str::FromStr;

        let mut result = HashMap::new();
        for line in SPRITES_DESCR.lines() {
            let words = line.split(" ").collect::<Vec<_>>();
            if words.len() != 6 {
                panic!("Can't parse sprite description");
            }
            let object = SpritesData::parse_sprite_name(words[0])
                .expect("Can't parse sprite's name");
            let offset_x = u32::from_str(words[1])
                .expect("Can't parse an int from sprite description");
            let offset_y = u32::from_str(words[2])
                .expect("Can't parse an int from sprite description");
            let width = u32::from_str(words[3])
                .expect("Can't parse an int from sprite description");
            let height = u32::from_str(words[4])
                .expect("Can't parse an int from sprite description");
            let frames_count = u32::from_str(words[5])
                .expect("Can't parse an int from sprite description");
            result.insert(object,
                          SpriteData {
                              image_offset: [offset_x as f32 / image_dimensions.0 as f32,
                                             (image_dimensions.1 - offset_y - height) as f32 /
                                             image_dimensions.1 as f32],
                              image_size: [width as f32 / image_dimensions.0 as f32,
                                           height as f32 / image_dimensions.1 as f32],
                              virtual_size: [width as f32 / virtual_dimensions.0 as f32,
                                             height as f32 / virtual_dimensions.1 as f32],
                              frames_count: frames_count,
                              hitbox: SpritesData::calculate_hitbox(image_buffer,
                                                                    (offset_x, offset_y),
                                                                    (width, height)),
                          });
        }

        return result;
    }

    fn calculate_hitbox(image_buffer: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
                        offset: (u32, u32),
                        size: (u32, u32))
                        -> Hitbox {
        use std::u8::MIN;
        use std::cmp;

        let mut left: u32 = size.0;
        let mut top: u32 = size.1;
        let mut right: u32 = 0;
        let mut bottom: u32 = 0;


        for x in 0..size.0 {
            for y in 0..size.1 {
                if image_buffer.get_pixel(offset.0 + x, offset.1 + y)[3] != MIN {
                    left = cmp::min(left, x);
                    top = cmp::min(top, y);
                    right = cmp::max(right, x);
                    bottom = cmp::max(bottom, y);
                }
            }
        }

        Hitbox {
            left: left as f32 / size.0 as f32,
            top: top as f32 / size.1 as f32,
            right: right as f32 / size.0 as f32,
            bottom: bottom as f32 / size.1 as f32,
        }
    }
}



impl SpritesData {
    pub fn new(virtual_dimensions: (u32, u32)) -> SpritesData {
        use std;

        let image_buffer = image::load(std::io::Cursor::new(SPRITES_IMAGE), image::PNG)
            .expect("Can't read png texture")
            .to_rgba();
        let image_dimensions = image_buffer.dimensions();
        SpritesData::from_image_buffer(image_buffer, virtual_dimensions, image_dimensions)
    }

    pub fn get_sprite_data(&self, object: SpriteObject) -> Option<&SpriteData> {
        self.sprites.get(&object)
    }

    pub fn get_image_buffer(&self) -> Vec<u8> {
        self.image_buffer.clone()
    }

    pub fn get_image_size(&self) -> (u32, u32) {
        self.image_size
    }
}
