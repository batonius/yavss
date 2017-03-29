use std::time::Duration;
use input::InputPoller;
use sprites::{SpriteObject, SpriteData, SpritesData};
use collision::{detect_collisions, CollisionData};
use util::{Angle, FPoint};

type CoordValue = f32;
type Speed = f32; //Screens/s

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

#[derive(Debug, Clone)]
pub struct SceneObject {
    pub object_type: ObjectType,
    pub pos: FPoint,
    pub direction_angle: Angle,
    pub to_delete: bool,

    sprite_angle: Angle,
    collision_data: CollisionData,
}

impl SceneObject {
    pub fn new<P>(sprites_data_cache: &SpriteDataCache,
                  object_type: ObjectType,
                  pos: P,
                  sprite_angle: Angle)
                  -> SceneObject
        where P: Into<FPoint>
    {
        SceneObject {
            object_type: object_type,
            pos: pos.into(),
            direction_angle: Angle::from_deg(sprite_angle.as_deg() - 90.0),
            to_delete: false,
            sprite_angle: sprite_angle,
            collision_data: CollisionData::new(sprites_data_cache.sprite_data(&object_type),
                                               sprite_angle),
        }
    }

    pub fn sprite_angle(&self) -> Angle {
        self.sprite_angle
    }

    pub fn collision_data(&self) -> &CollisionData {
        &self.collision_data
    }

    pub fn set_sprite_angle(&mut self, sprites_data_cache: &SpriteDataCache, sprite_angle: Angle) {
        self.collision_data = CollisionData::new(sprites_data_cache.sprite_data(&self.object_type),
                                                 sprite_angle);
        self.sprite_angle = sprite_angle;
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

pub struct SceneIterator<'a> {
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
            self.player_bullets
                .next()
                .or_else(|| {
                             self.enemy_bullets
                                 .next()
                                 .or_else(|| {
                                              self.empty = true;
                                              Some(self.player)
                                          })
                         })
        }
    }
}

#[derive(Debug)]
pub struct SpriteDataCache<'a> {
    player_sprite_data: &'a SpriteData,
    player_bullet_sprite_data: &'a SpriteData,
    enemy_bullet_sprite_data: &'a SpriteData,
}

impl<'a> SpriteDataCache<'a> {
    pub fn new(sprites_data: &'a SpritesData) -> SpriteDataCache<'a> {
        SpriteDataCache {
            player_sprite_data: sprites_data.sprite_data(SpriteObject::Player).unwrap(),
            player_bullet_sprite_data: sprites_data
                .sprite_data(SpriteObject::PlayerBullet)
                .unwrap(),
            enemy_bullet_sprite_data: sprites_data
                .sprite_data(SpriteObject::EnemyBullet)
                .unwrap(),
        }
    }

    pub fn sprite_data(&self, object_type: &ObjectType) -> &'a SpriteData {
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
        let sprite_data_cache = SpriteDataCache::new(sprites_data);
        let player_scene_object = SceneObject::new(&sprite_data_cache,
                                                   ObjectType::Player(PlayerState::Normal),
                                                   (0.5, 0.8),
                                                   Angle::from_deg(0.0));
        Scene {
            speeds: speeds,
            background_position: 0.0,
            player_scene_object: player_scene_object,
            bullets_frame: 0.0,
            sprites_data: sprites_data,
            firing_timeout: bullets_timeout,
            player_bullets: vec![],
            enemy_bullets: vec![],
            new_bullet_timeout: bullets_timeout,
            sprite_data_cache: sprite_data_cache,
        }
    }

    pub fn total_objects(&self) -> usize {
        self.player_bullets.len() + self.enemy_bullets.len() + 1
    }

    pub fn background_position(&self) -> f32 {
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

    pub fn objects(&self) -> SceneIterator {
        SceneIterator::new(self)
    }

    fn add_bullets(&mut self, duration_s: f32) {
        self.new_bullet_timeout += duration_s;
        if self.new_bullet_timeout >= self.speeds.bullet_shooting_speed * 3.0 {
            self.new_bullet_timeout = 0.0;
            self.enemy_bullets
                .push(SceneObject::new(&self.sprite_data_cache,
                                       ObjectType::EnemyBullet(0),
                                       (self.player_scene_object.pos.x(), MIN_Y_VALUE),
                                       Angle::from_deg(-180.0)));
            // let bullets_count = 3200;
            // for x in 0..bullets_count {
            //     self.enemy_bullets.push(SceneObject::new(&self.sprite_data_cache,
            //                                              ObjectType::EnemyBullet(0),
            //                                              (x as f32 / bullets_count as f32,
            //                                               MAX_Y_VALUE),
            //                                              Angle::from_deg(-180.0)));
            //     self.player_bullets.push(SceneObject::new(&self.sprite_data_cache,
            //                                               ObjectType::PlayerBullet(0),
            //                                               (x as f32 / bullets_count as f32,
            //                                                MIN_Y_VALUE),
            //                                               Angle::from_deg(0.0)));
            // }
        }
    }

    fn process_input(&mut self, input: &InputPoller, duration_s: f32) {
        self.firing_timeout += duration_s;
        if input.fire_is_pressed() && self.firing_timeout >= self.speeds.bullet_shooting_speed {
            self.firing_timeout = 0.0;
            let adjusted_angle = self.player_scene_object.sprite_angle();
            self.player_bullets
                .push(SceneObject::new(&self.sprite_data_cache,
                                       ObjectType::PlayerBullet(0),
                                       self.player_scene_object.pos,
                                       adjusted_angle));
            self.player_bullets
                .push(SceneObject::new(&self.sprite_data_cache,
                                       ObjectType::PlayerBullet(0),
                                       self.player_scene_object.pos,
                                       adjusted_angle.add_deg(20.0)));
            self.player_bullets
                .push(SceneObject::new(&self.sprite_data_cache,
                                       ObjectType::PlayerBullet(0),
                                       self.player_scene_object.pos,
                                       adjusted_angle.add_deg(-20.0)));
        }
    }

    fn move_player(&mut self, input: &InputPoller, duration_s: f32) {
        let (mut x, mut y) = self.player_scene_object.pos.into();
        let x_move = input.x_move();
        {
            let player_virtual_hitbox = self.player_scene_object.collision_data().hitbox();
            x += x_move * self.speeds.x_speed * (duration_s as CoordValue);
            x = x.min(MAX_X_VALUE - player_virtual_hitbox.right)
                .max(MIN_X_VALUE + player_virtual_hitbox.left);
            y += input.y_move() * self.speeds.y_speed * (duration_s as CoordValue);
            y = y.min(MAX_Y_VALUE - player_virtual_hitbox.bottom)
                .max(MIN_Y_VALUE + player_virtual_hitbox.top);
        }
        self.player_scene_object.pos = FPoint::new(x, y);
        if x_move < -0.2 {
            self.player_scene_object.object_type = ObjectType::Player(PlayerState::TiltedLeft);
        } else if x_move > 0.2 {
            self.player_scene_object.object_type = ObjectType::Player(PlayerState::TiltedRight);
        } else {
            self.player_scene_object.object_type = ObjectType::Player(PlayerState::Normal);
        }

        let total_tilt = (input.right_tilt() - input.left_tilt()) * 90.0;
        if (total_tilt - self.player_scene_object.sprite_angle().as_deg()).abs() > 1.0 {
            self.player_scene_object
                .set_sprite_angle(&self.sprite_data_cache, Angle::from_deg(total_tilt));
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
        for object in objects.iter_mut() {
            let distance = speed * duration_s;
            let direction_angle = object.direction_angle.as_rad();
            *object.pos.mut_x() += direction_angle.cos() * distance;
            *object.pos.mut_y() += direction_angle.sin() * distance;
        }
        objects.retain(|object| {
                           object.pos.x() >= MIN_X_VALUE || object.pos.x() <= MAX_X_VALUE ||
                           object.pos.y() >= MIN_Y_VALUE ||
                           object.pos.y() <= MAX_Y_VALUE
                       });
    }

    fn detect_collisions(&mut self) {
        use std::iter;

        detect_collisions(&mut self.enemy_bullets,
                          iter::once(&mut self.player_scene_object),
                          |a, _| { a.to_delete = true; });
        detect_collisions(&mut self.enemy_bullets, &mut self.player_bullets, |a, b| {
            a.to_delete = true;
            b.to_delete = true;
        });
        self.enemy_bullets.retain(|bullet| !bullet.to_delete);
        self.player_bullets.retain(|bullet| !bullet.to_delete);
    }
}
