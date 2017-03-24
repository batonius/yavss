use image;
use std::collections::HashMap;
use std::collections::hash_map::Keys;
use ::sprites::convex;

const SPRITES_IMAGE: &'static [u8] = include_bytes!("../../data/sprites.png");
const SPRITES_DESCR: &'static str = include_str!("../../data/sprites.txt");

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum SpriteObject {
    Background,
    Player,
    PlayerBullet,
    EnemyBullet,
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
    convex: Vec<(f32, f32)>,
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
            "PLAYER_BULLET" => Some(SpriteObject::PlayerBullet),
            "ENEMY_BULLET" => Some(SpriteObject::EnemyBullet),
            _ => None,
        }
    }

    fn parse_sprites_descr(image_buffer: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
                           virtual_dimensions: (u32, u32),
                           image_dimensions: (u32, u32))
                           -> HashMap<SpriteObject, SpriteData> {
        use std::str::FromStr;
        use std::cmp::{min, max};

        let mut result = HashMap::new();
        for line in SPRITES_DESCR.lines() {
            if line.is_empty() {
                continue;
            }
            let words = line.split(' ').collect::<Vec<_>>();
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

            let half_pixel_widht = (1.0 / width as f32) / 2.0;
            let half_pixel_height = (1.0 / height as f32) / 2.0;
            let convex =
                convex::calculate_convex(image_buffer, (offset_x, offset_y), (width, height));

            let (left, top, right, bottom) = convex.iter().fold((width - 1, height - 1, 0, 0),
                                                                |(left, top, right, bottom),
                                                                 &(x, y)| {
                let x = x as u32;
                let y = y as u32;
                (min(left, x), min(top, y), max(right, x), max(bottom, y))
            });
            let hitbox = Hitbox {
                left: left as f32 / width as f32,
                top: top as f32 / height as f32,
                right: (right + 1) as f32 / width as f32,
                bottom: (bottom + 1) as f32 / height as f32,
            };
            let convex = convex.into_iter()
                .map(|(x, y)| {
                    (x as f32 / width as f32 + half_pixel_widht,
                     (height as i32 - y) as f32 / height as f32 + half_pixel_height)
                })
                .collect();

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
                              hitbox: hitbox,
                              convex: convex,
                          });
        }

        result
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

    pub fn get_sprite_objects(&self) -> Keys<SpriteObject, SpriteData> {
        self.sprites.keys()
    }
}
