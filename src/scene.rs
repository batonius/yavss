use std::time::Duration;
use input::InputPoller;
use sprites_data::{SpriteObject, SpriteData, SpritesData};

type CoordValue = f32;
type Speed = f32; //Screens/s
type Position = (CoordValue, CoordValue);

const MIN_X_VALUE: CoordValue = 0.0;
const MAX_X_VALUE: CoordValue = 1.0;
const MIN_Y_VALUE: CoordValue = 0.0;
const MAX_Y_VALUE: CoordValue = 1.0;

#[derive(Debug, Clone, Copy)]
pub enum PlayerState {
    Normal,
    TiltedLeft,
    TiltedRight,
}

#[derive(Debug, Clone, Copy)]
pub enum ObjectType {
    Player(PlayerState),
    PlayerBullet(u32),
    EnemyBullet(u32),
}

#[derive(Debug, Clone, Copy)]
pub struct SceneObject {
    pub object_type: ObjectType,
    pub pos: Position,
    pub angle: f32,
}

impl SceneObject {
    pub fn new(object_type: ObjectType, pos: Position, angle: f32) -> SceneObject {
        SceneObject {
            object_type: object_type,
            pos: pos,
            angle: angle,
        }
    }
}

#[derive(Debug)]
pub struct SpeedValues {
    background_speed: Speed,
    x_speed: Speed,
    y_speed: Speed,
    bullet_blicking_speed: Speed,
    bullet_speed: Speed,
    bullet_shooting_speed: Speed,
}

impl Default for SpeedValues {
    fn default() -> SpeedValues {
        SpeedValues {
            x_speed: 0.75,
            y_speed: 0.75,
            background_speed: 0.15,
            bullet_blicking_speed: 2.0,
            bullet_speed: 0.5,
            bullet_shooting_speed: 0.2,
        }
    }
}

pub struct SceneIterator<'a>
{
    player_bullets: <&'a Vec<SceneObject> as IntoIterator>::IntoIter,
    enemy_bullets: <&'a Vec<SceneObject> as IntoIterator>::IntoIter,
    player: &'a SceneObject,
    empty: bool,
}

impl<'a> SceneIterator<'a> {
    pub fn new<'b>(scene: &'a Scene<'b>) -> SceneIterator<'a>
        where 'b: 'a
    {
        SceneIterator {
            player_bullets: scene.player_bullets.iter(),
            enemy_bullets: scene.enemy_bullets.iter(),
            player: &scene.player_scene_object,
            empty: false,
        }
    }
}

impl<'a> Iterator for SceneIterator<'a> {
    type Item = &'a SceneObject;

    fn next(&mut self) -> Option<Self::Item> {
        if self.empty {
            None
        } else {
            self.player_bullets.next().or_else(|| {
                self.enemy_bullets.next().or_else(|| {
                    self.empty = true;
                    Some(self.player)
                })
            })
        }
    }
}

#[derive(Debug)]
struct SpriteDataCache<'a> {
    player_sprite_data: &'a SpriteData,
    player_bullet_sprite_data: &'a SpriteData,
    enemy_bullet_sprite_data: &'a SpriteData,
}

impl<'a> SpriteDataCache<'a> {
    pub fn new(sprites_data: &'a SpritesData) -> SpriteDataCache<'a> {
        SpriteDataCache {
            player_sprite_data: sprites_data.get_sprite_data(SpriteObject::Player).unwrap(),
            player_bullet_sprite_data: sprites_data.get_sprite_data(SpriteObject::PlayerBullet)
                .unwrap(),
            enemy_bullet_sprite_data: sprites_data.get_sprite_data(SpriteObject::EnemyBullet)
                .unwrap(),
        }
    }

    pub fn get_sprite_data(&self, object_type: &ObjectType) -> &'a SpriteData {
        match *object_type {
            ObjectType::Player(..) => self.player_sprite_data,
            ObjectType::EnemyBullet(..) => self.enemy_bullet_sprite_data,
            ObjectType::PlayerBullet(..) => self.player_bullet_sprite_data,
        }
    }
}

#[derive(Debug)]
pub struct Scene<'a> {
    sprites_data: &'a SpritesData,
    speeds: SpeedValues,
    background_position: f32,
    player_scene_object: SceneObject,
    bullets_frame: f32,
    firing_timeout: f32,
    player_bullets: Vec<SceneObject>,
    enemy_bullets: Vec<SceneObject>,
    new_bullet_timeout: f32,
    sprite_data_cache: SpriteDataCache<'a>,
}

impl<'a> Scene<'a> {
    pub fn new(sprites_data: &'a SpritesData) -> Scene<'a> {
        let speeds = SpeedValues::default();
        let bullets_timeout = speeds.bullet_shooting_speed + 1.0;
        Scene {
            speeds: speeds,
            background_position: 0.0,
            player_scene_object: SceneObject::new(ObjectType::Player(PlayerState::Normal),
                                                  (0.5, 0.2),
                                                  0.0),
            bullets_frame: 0.0,
            sprites_data: sprites_data,
            firing_timeout: bullets_timeout,
            player_bullets: vec![],
            enemy_bullets: vec![],
            new_bullet_timeout: bullets_timeout,
            sprite_data_cache: SpriteDataCache::new(sprites_data),
        }
    }

    pub fn get_background_position(&self) -> f32 {
        self.background_position
    }

    pub fn tick(&mut self, input: &InputPoller, duration: Duration) {
        let duration_s = (duration.as_secs() as f32) +
                         (duration.subsec_nanos() as f32 / 1_000_000_000f32);
        self.process_input(input, duration_s);
        self.move_player(input, duration_s);
        self.move_background(duration_s);
        self.add_bullets(duration_s);
        self.move_bullets(duration_s);
        self.detect_collisions();
        self.blink_bullet(duration_s);
    }

    pub fn get_objects(&self) -> SceneIterator {
        SceneIterator::new(self)
    }

    fn add_bullets(&mut self, duration_s: f32) {
        self.new_bullet_timeout += duration_s;
        if self.new_bullet_timeout >= self.speeds.bullet_shooting_speed * 3.0 {
            self.new_bullet_timeout = 0.0;
            self.enemy_bullets.push(SceneObject::new(ObjectType::EnemyBullet(0),
                                                     (self.player_scene_object.pos.0, MAX_Y_VALUE),
                                                     -180.0))
        }
    }

    fn process_input(&mut self, input: &InputPoller, duration_s: f32) {
        self.firing_timeout += duration_s;
        if input.fire_is_pressed() && self.firing_timeout >= self.speeds.bullet_shooting_speed {
            self.firing_timeout = 0.0;
            self.player_bullets.push(SceneObject::new(ObjectType::PlayerBullet(0),
                                                      self.player_scene_object.pos,
                                                      0.0f32));
            self.player_bullets.push(SceneObject::new(ObjectType::PlayerBullet(0),
                                                      self.player_scene_object.pos,
                                                      -20.0f32));
            self.player_bullets.push(SceneObject::new(ObjectType::PlayerBullet(0),
                                                      self.player_scene_object.pos,
                                                      20.0f32));
        }
    }

    fn move_player(&mut self, input: &InputPoller, duration_s: f32) {
        let player_virtual_hitbox = self.sprite_data_cache
            .get_sprite_data(&self.player_scene_object.object_type)
            .get_virtual_hitbox();
        let (mut x, mut y) = self.player_scene_object.pos;
        let x_move = input.x_move();
        x += x_move * self.speeds.x_speed * (duration_s as CoordValue);
        x = x.min(MAX_X_VALUE - player_virtual_hitbox.right)
            .max(MIN_X_VALUE + player_virtual_hitbox.left);
        y += input.y_move() * self.speeds.y_speed * (duration_s as CoordValue);
        y = y.min(MAX_Y_VALUE - player_virtual_hitbox.top)
            .max(MIN_Y_VALUE + player_virtual_hitbox.bottom);
        self.player_scene_object.pos = (x, y);
        if x_move < -0.2 {
            self.player_scene_object.object_type = ObjectType::Player(PlayerState::TiltedLeft);
        } else if x_move > 0.2 {
            self.player_scene_object.object_type = ObjectType::Player(PlayerState::TiltedRight);
        } else {
            self.player_scene_object.object_type = ObjectType::Player(PlayerState::Normal);
        }
    }

    fn move_background(&mut self, duration_s: f32) {
        self.background_position += self.speeds.background_speed * (duration_s as f32);
    }

    fn blink_bullet(&mut self, duration_s: f32) {
        self.bullets_frame += self.speeds.bullet_blicking_speed * (duration_s as f32);
        let iter = (&mut self.player_bullets).iter_mut();
        let iter = iter.chain((&mut self.enemy_bullets).iter_mut());
        for bullet in iter {
            bullet.object_type = match bullet.object_type {
                ObjectType::PlayerBullet(_) => ObjectType::PlayerBullet(self.bullets_frame as u32),
                ObjectType::EnemyBullet(_) => ObjectType::EnemyBullet(self.bullets_frame as u32),
                object_type @ _ => object_type,
            }
        }
    }

    fn move_bullets(&mut self, duration_s: f32) {
        Scene::move_objects(&mut self.player_bullets,
                            self.speeds.bullet_speed,
                            duration_s);
        Scene::move_objects(&mut self.enemy_bullets,
                            self.speeds.bullet_speed,
                            duration_s);
    }

    fn move_objects(objects: &mut Vec<SceneObject>, speed: f32, duration_s: f32) {
        use std::f32;
        for object in objects.iter_mut() {
            let distance = speed * duration_s;
            let angle_rad = (-object.angle + 90.0) / 180.0 * f32::consts::PI;
            object.pos.0 += angle_rad.cos() * distance;
            object.pos.1 += angle_rad.sin() * distance;
        }
        objects.retain(|&object| {
            object.pos.0 >= MIN_X_VALUE || object.pos.0 <= MAX_X_VALUE ||
            object.pos.1 >= MIN_Y_VALUE || object.pos.1 <= MAX_Y_VALUE
        });
    }

    fn detect_collisions(&mut self) {
        let player_scene_object = &self.player_scene_object;
        let sprite_data_cache = &self.sprite_data_cache;
        self.enemy_bullets.retain(|bullet| {
            !Scene::check_collision(sprite_data_cache, bullet, player_scene_object)
        });
    }

    fn check_collision(sprite_data_cache: &SpriteDataCache,
                       a: &SceneObject,
                       b: &SceneObject)
                       -> bool {
        let a_hitbox = sprite_data_cache.get_sprite_data(&a.object_type).get_virtual_hitbox();
        let b_hitbox = sprite_data_cache.get_sprite_data(&b.object_type).get_virtual_hitbox();
        if a.pos.0 + a_hitbox.right > b.pos.0 - b_hitbox.left &&
           b.pos.0 + b_hitbox.right > a.pos.0 - a_hitbox.left &&
           a.pos.1 + a_hitbox.top > b.pos.1 - b_hitbox.bottom &&
           b.pos.1 + b_hitbox.top > a.pos.1 - a_hitbox.bottom {
            true
        } else {
            false
        }
    }
}
