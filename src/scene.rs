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

#[derive(Debug)]
pub enum ObjectType {
    Player(PlayerState),
    Bullet(u32),
}

#[derive(Debug)]
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
}

impl SpeedValues {
    pub fn new(x_speed: Speed,
               y_speed: Speed,
               background_speed: Speed,
               bullet_blicking_speed: Speed)
               -> SpeedValues {
        SpeedValues {
            x_speed: x_speed,
            y_speed: y_speed,
            background_speed: background_speed,
            bullet_blicking_speed: bullet_blicking_speed,
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
        }
    }

    pub fn background_position(&self) -> f32 {
        self.background_position
    }

    pub fn tick(&mut self, input: &InputPoller, duration: Duration) {
        let duration_s = (duration.as_secs() as f32) +
                         (duration.subsec_nanos() as f32 / 1_000_000_000f32);
        self.move_player(input, duration_s);
        self.move_background(duration_s);
        self.blink_bullet(duration_s);
    }

    pub fn get_scene_objects(&self) -> Vec<SceneObject> {
        let result = vec![SceneObject {
                              object_type: ObjectType::Player(self.player_state),
                              pos: self.player_position,
                              angle: -self.bullets_frame * 30.0,
                          },
                          SceneObject {
                              object_type: ObjectType::Bullet((self.bullets_frame as u32) % 4),
                              pos: (0.5, 0.7),
                              angle: self.bullets_frame * 30.0,
                          }];
        return result;
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
}
