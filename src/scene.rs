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
}

impl SpeedValues {
    pub fn new(x_speed: Speed,
               y_speed: Speed,
               background_speed: Speed,
               bullet_blicking_speed: Speed,
               bullet_speed: Speed)
               -> SpeedValues {
        SpeedValues {
            x_speed: x_speed,
            y_speed: y_speed,
            background_speed: background_speed,
            bullet_blicking_speed: bullet_blicking_speed,
            bullet_speed: bullet_speed,
        }
    }
}

#[derive(Debug)]
pub struct Scene<'a> {
    sprites_data: &'a SpritesData,
    speeds: SpeedValues,
    background_position: f32,
    player_position: Position,
    player_state: PlayerState,
    bullets_frame: f32,
    bullets: Vec<SceneObject>,
}

impl<'a> Scene<'a> {
    pub fn new(speeds: SpeedValues, sprites_data: &'a SpritesData) -> Scene<'a> {
        Scene {
            speeds: speeds,
            background_position: 0.0,
            player_position: (0.5, 0.2),
            player_state: PlayerState::Normal,
            bullets_frame: 0.0,
            sprites_data: sprites_data,
            bullets: vec![],
        }
    }

    pub fn background_position(&self) -> f32 {
        self.background_position
    }

    pub fn tick(&mut self, input: &InputPoller, duration: Duration) {
        let duration_s = (duration.as_secs() as f32) +
                         (duration.subsec_nanos() as f32 / 1_000_000_000f32);
        self.process_input(input);
        self.move_player(input, duration_s);
        self.move_background(duration_s);
        self.move_bullets(duration_s);
        self.blink_bullet(duration_s);
    }

    pub fn get_scene_objects(&self) -> Vec<SceneObject> {
        let mut result = self.bullets.clone();
        result.push(SceneObject {
            object_type: ObjectType::Player(self.player_state),
            pos: self.player_position,
            angle: 0.0,
        });
        result
    }

    fn process_input(&mut self, input: &InputPoller) {
        if input.fire_is_pressed() {
            self.bullets.push(SceneObject {
                object_type: ObjectType::Bullet(0),
                pos: self.player_position,
                angle: 0.0f32,
            });
            self.bullets.push(SceneObject {
                object_type: ObjectType::Bullet(0),
                pos: self.player_position,
                angle: -20.0f32,
            });
            self.bullets.push(SceneObject {
                object_type: ObjectType::Bullet(0),
                pos: self.player_position,
                angle: 20.0f32,
            });
        }
    }

    fn move_player(&mut self, input: &InputPoller, duration_s: f32) {
        let player_virtual_size = self.sprites_data
            .get_sprite_data(SpriteObject::Player)
            .expect("Can't get player sprite")
            .get_virtual_size();
        let (mut x, mut y) = self.player_position;
        let x_move = input.x_move();
        x += x_move * self.speeds.x_speed * (duration_s as CoordValue);
        x = x.min(MAX_X_VALUE - player_virtual_size[0] / 2.0)
            .max(MIN_X_VALUE + player_virtual_size[0] / 2.0);
        y += input.y_move() * self.speeds.y_speed * (duration_s as CoordValue);
        y = y.min(MAX_Y_VALUE - player_virtual_size[1] / 2.0)
            .max(MIN_Y_VALUE + player_virtual_size[1] / 2.0);
        self.player_position = (x, y);
        if x_move < -0.2 {
            self.player_state = PlayerState::TiltedLeft;
        } else if x_move > 0.2 {
            self.player_state = PlayerState::TiltedRight;
        } else {
            self.player_state = PlayerState::Normal;
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
