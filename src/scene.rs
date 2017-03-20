use std::time::Duration;
use input::InputPoller;
use sprites_data::{SpriteObject, SpritesData};

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
    Bullet(u32),
}

#[derive(Debug, Clone, Copy)]
pub struct SceneObject {
    pub object_type: ObjectType,
    pub pos: Position,
    pub angle: f32,
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

#[derive(Debug)]
pub struct Scene<'a> {
    sprites_data: &'a SpritesData,
    speeds: SpeedValues,
    background_position: f32,
    player_scene_object: SceneObject,
    bullets_frame: f32,
    bullets_timeout: f32,
    bullets: Vec<SceneObject>,
}

pub struct SceneIterator<'a>
{
    bullets: <&'a Vec<SceneObject> as IntoIterator>::IntoIter,
    player: &'a SceneObject,
    empty: bool,
}

impl<'a> SceneIterator<'a>
{
    pub fn new<'b>(scene: &'a Scene<'b>) -> SceneIterator<'a>
        where 'b: 'a
    {
        SceneIterator {
            bullets: scene.bullets.iter(),
            player: &scene.player_scene_object,
            empty: false,
        }
    }
}

impl<'a> Iterator for SceneIterator<'a>
{
    type Item = &'a SceneObject;

    fn next(&mut self) -> Option<Self::Item> {
        if self.empty {
            None
        } else {
            self.bullets.next().or_else(|| {
                self.empty = true;
                Some(self.player)
            })
        }
    }
}

impl<'a> Scene<'a> {
    pub fn new(speeds: SpeedValues, sprites_data: &'a SpritesData) -> Scene<'a> {
        let bullets_timeout = speeds.bullet_shooting_speed + 1.0;
        Scene {
            speeds: speeds,
            background_position: 0.0,
            player_scene_object: SceneObject {
                object_type: ObjectType::Player(PlayerState::Normal),
                pos: (0.5, 0.2),
                angle: 0.0,
            },
            bullets_frame: 0.0,
            sprites_data: sprites_data,
            bullets_timeout: bullets_timeout,
            bullets: vec![],
        }
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
        self.move_bullets(duration_s);
        self.blink_bullet(duration_s);
    }

    pub fn objects<'c>(&'c self) -> SceneIterator<'c> {
        SceneIterator::new(self)
    }

    fn process_input(&mut self, input: &InputPoller, duration_s: f32) {
        self.bullets_timeout += duration_s;
        if input.fire_is_pressed() && self.bullets_timeout >= self.speeds.bullet_shooting_speed {
            self.bullets_timeout = 0.0;
            self.bullets.push(SceneObject {
                object_type: ObjectType::Bullet(0),
                pos: self.player_scene_object.pos,
                angle: 0.0f32,
            });
            self.bullets.push(SceneObject {
                object_type: ObjectType::Bullet(0),
                pos: self.player_scene_object.pos,
                angle: -20.0f32,
            });
            self.bullets.push(SceneObject {
                object_type: ObjectType::Bullet(0),
                pos: self.player_scene_object.pos,
                angle: 20.0f32,
            });
        }
    }

    fn move_player(&mut self, input: &InputPoller, duration_s: f32) {
        let player_virtual_size = self.sprites_data
            .get_sprite_data(SpriteObject::Player)
            .expect("Can't get player sprite")
            .get_virtual_size();
        let (mut x, mut y) = self.player_scene_object.pos;
        let x_move = input.x_move();
        x += x_move * self.speeds.x_speed * (duration_s as CoordValue);
        x = x.min(MAX_X_VALUE - player_virtual_size[0] / 2.0)
            .max(MIN_X_VALUE + player_virtual_size[0] / 2.0);
        y += input.y_move() * self.speeds.y_speed * (duration_s as CoordValue);
        y = y.min(MAX_Y_VALUE - player_virtual_size[1] / 2.0)
            .max(MIN_Y_VALUE + player_virtual_size[1] / 2.0);
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
    }

    fn move_bullets(&mut self, duration_s: f32) {
        use std::f32;
        for bullet in &mut self.bullets {
            let distance = self.speeds.bullet_speed * duration_s;
            let angle_rad = (-bullet.angle + 90.0) / 180.0 * f32::consts::PI;
            bullet.pos.0 += angle_rad.cos() * distance;
            bullet.pos.1 += angle_rad.sin() * distance;
        }
        self.bullets.retain(|&bullet| {
            bullet.pos.0 >= MIN_X_VALUE || bullet.pos.0 <= MAX_X_VALUE ||
            bullet.pos.1 >= MIN_Y_VALUE || bullet.pos.1 <= MAX_Y_VALUE
        });
    }
}
