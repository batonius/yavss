use image;
use std::collections::HashMap;
use std::collections::hash_map::Keys;
use sprites::convex;
use util::{IPoint, UPoint, FPoint, FDimensions, Dimensions};

const SPRITES_IMAGE: &'static [u8] = include_bytes!("../../data/sprites.png");
const SPRITES_DESCR: &'static str = include_str!("../../data/sprites.txt");

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum SpriteObject {
    Background,
    Player,
    PlayerBullet,
    EnemyBullet,
}

#[derive(Debug)]
pub struct SpriteData {
    image_offset: FPoint,
    image_size: FDimensions,
    virtual_size: FDimensions,
    frames_count: u32,
    convex: Vec<FPoint>,
}

impl SpriteData {
    pub fn image_offset(&self) -> FPoint {
        self.image_offset
    }

    pub fn image_size(&self) -> FDimensions {
        self.image_size
    }

    pub fn virtual_size(&self) -> FDimensions {
        self.virtual_size
    }

    pub fn frames_count(&self) -> u32 {
        self.frames_count
    }

    pub fn convex(&self) -> &Vec<FPoint> {
        &self.convex
    }
}

#[derive(Debug)]
pub struct SpritesData {
    image_buffer: Vec<u8>,
    image_size: Dimensions,
    sprites: HashMap<SpriteObject, SpriteData>,
}

impl SpritesData {
    pub fn from_image_buffer<D1, D2>(image_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
                                     virtual_dimensions: D1,
                                     image_dimensions: D2)
                                     -> SpritesData
        where D1: Into<Dimensions>,
              D2: Into<Dimensions>
    {
        let image_dimensions = image_dimensions.into();
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

    fn parse_sprites_descr<D1, D2>(image_buffer: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
                                   virtual_dimensions: D1,
                                   image_dimensions: D2)
                                   -> HashMap<SpriteObject, SpriteData>
        where D1: Into<Dimensions>,
              D2: Into<Dimensions>
    {
        use std::str::FromStr;

        let image_dimensions = image_dimensions.into();
        let virtual_dimensions = virtual_dimensions.into().as_f32();

        let mut result = HashMap::new();
        for line in SPRITES_DESCR.lines() {
            if line.is_empty() {
                continue;
            }
            let words = line.split(' ').collect::<Vec<_>>();
            if words.len() != 6 {
                panic!("Can't parse sprite description");
            }
            let object =
                SpritesData::parse_sprite_name(words[0]).expect("Can't parse sprite's name");
            let offset_x =
                u32::from_str(words[1]).expect("Can't parse an int from sprite description");
            let offset_y =
                u32::from_str(words[2]).expect("Can't parse an int from sprite description");
            let width =
                u32::from_str(words[3]).expect("Can't parse an int from sprite description");
            let height =
                u32::from_str(words[4]).expect("Can't parse an int from sprite description");
            let frames_count =
                u32::from_str(words[5]).expect("Can't parse an int from sprite description");

            let convex =
                convex::calculate_convex(image_buffer, (offset_x, offset_y), (width, height));

            let image_size = FPoint::new(width as f32, height as f32);

            let convex = convex
                .into_iter()
                .map(|(p, d)| {
                         IPoint::new(p.x() + d.x() - width as i32 / 2,
                                     height as i32 / 2 - p.y() - d.y())
                                 .as_f32() / virtual_dimensions
                     })
                .collect();

            result.insert(object,
                          SpriteData {
                              image_offset: FPoint::new(offset_x as f32,
                                                        (image_dimensions.y() - offset_y -
                                                         height) as
                                                        f32) /
                                            image_dimensions.as_f32(),
                              image_size: image_size / image_dimensions.as_f32(),
                              virtual_size: image_size / virtual_dimensions,
                              frames_count: frames_count,
                              convex: convex,
                          });
        }

        result
    }
}

impl SpritesData {
    pub fn new<D>(virtual_dimensions: D) -> SpritesData
        where D: Into<Dimensions>
    {
        use std;

        let image_buffer = image::load(std::io::Cursor::new(SPRITES_IMAGE), image::PNG)
            .expect("Can't read png texture")
            .to_rgba();
        let image_dimensions: UPoint = image_buffer.dimensions().into();
        SpritesData::from_image_buffer(image_buffer, virtual_dimensions, image_dimensions)
    }

    pub fn sprite_data(&self, object: SpriteObject) -> Option<&SpriteData> {
        self.sprites.get(&object)
    }

    pub fn image_buffer(&self) -> Vec<u8> {
        self.image_buffer.clone()
    }

    pub fn image_size(&self) -> Dimensions {
        self.image_size
    }

    pub fn sprite_objects(&self) -> Keys<SpriteObject, SpriteData> {
        self.sprites.keys()
    }
}
