use std::time::Duration;
use input::InputPoller;

type CoordValue = f32;
type Speed = f32; //Screens/s
type Position = (CoordValue, CoordValue);

const MIN_X_VALUE: CoordValue = 0.0;
const MAX_X_VALUE: CoordValue = 1.0;
const MIN_Y_VALUE: CoordValue = 0.0;
const MAX_Y_VALUE: CoordValue = 1.0;

#[derive(Debug)]
pub struct SpeedValues {
    background_speed: Speed,
    x_speed: Speed,
    y_speed: Speed,
}

impl SpeedValues {
    pub fn new(x_speed: Speed, y_speed: Speed, background_speed: Speed) -> SpeedValues {
        SpeedValues {
            x_speed: x_speed,
            y_speed: y_speed,
            background_speed: background_speed,
        }
    }
}

#[derive(Debug)]
pub struct Scene {
    speeds: SpeedValues,
    background_position: f32,
    player_position: Position,
}

impl Scene {
    pub fn new(speeds: SpeedValues) -> Scene {
        Scene {
            speeds: speeds,
            background_position: 0.0,
            player_position: (0.0, 0.0),
        }
    }

    pub fn player_position(&self) -> Position {
        self.player_position
    }

    pub fn background_position(&self) -> f32 {
        self.background_position
    }

    pub fn tick(&mut self, input: &InputPoller, duration: Duration) {
        let duration_s = (duration.as_secs() as f32) +
                         (duration.subsec_nanos() as f32 / 1_000_000_000f32);
        self.move_player(input, duration_s);
        self.move_background(duration_s);
    }

    fn move_player(&mut self, input: &InputPoller, duration_s: f32) {
        let (mut x, mut y) = self.player_position;
        x += input.x_move() * self.speeds.x_speed * (duration_s as CoordValue);
        x = x.min(MAX_X_VALUE).max(MIN_X_VALUE);
        y += input.y_move() * self.speeds.y_speed * (duration_s as CoordValue);
        y = y.min(MAX_Y_VALUE).max(MIN_Y_VALUE);
        self.player_position = (x, y);
    }

    fn move_background(&mut self, duration_s: f32) {
        self.background_position += self.speeds.background_speed * (duration_s as f32);
    }
}
