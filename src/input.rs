use glium::backend::glutin_backend::WinRef;
use gilrs::Gilrs;

type AxisValue = f32;

struct InputState {
    x_move: AxisValue,
    y_move: AxisValue,
    fire_is_pressed: bool,
    exit: bool,
    left_is_pressed: bool,
    right_is_pressed: bool,
    down_is_pressed: bool,
    up_is_pressed: bool,
    left_tilt: AxisValue,
    right_tilt: AxisValue,
}

impl Default for InputState {
    fn default() -> InputState {
        InputState {
            x_move: 0.0,
            y_move: 0.0,
            fire_is_pressed: false,
            exit: false,
            left_is_pressed: false,
            right_is_pressed: false,
            down_is_pressed: false,
            up_is_pressed: false,
            left_tilt: 0.0,
            right_tilt: 0.0,
        }
    }
}

impl InputState {
    fn clear(&mut self) {
        *self = Default::default();
    }
}

pub struct InputPoller<'a> {
    state: InputState,
    gilrs: Gilrs,
    win: WinRef<'a>,
}

impl<'a> InputPoller<'a> {
    pub fn new(win: WinRef<'a>) -> InputPoller<'a> {
        InputPoller {
            state: Default::default(),
            gilrs: Gilrs::new(),
            win: win,
        }
    }

    pub fn poll_events(&mut self) {
        use glium::glutin;
        use glium::glutin::Event;
        use gilrs;

        self.state.clear();

        for event in self.win.poll_events() {
            match event {
                Event::KeyboardInput(glutin::ElementState::Released, _, Some(key_code)) => {
                    match key_code {
                        glutin::VirtualKeyCode::W => {
                            self.state.up_is_pressed = false;
                        }
                        glutin::VirtualKeyCode::A => {
                            self.state.left_is_pressed = false;
                        }
                        glutin::VirtualKeyCode::S => {
                            self.state.down_is_pressed = false;
                        }
                        glutin::VirtualKeyCode::D => {
                            self.state.right_is_pressed = false;
                        }
                        glutin::VirtualKeyCode::Space => {
                            self.state.fire_is_pressed = false;
                        }
                        _ => {}
                    }
                }
                Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(key_code)) => {
                    match key_code {
                        glutin::VirtualKeyCode::Escape => {
                            self.state.exit = true;
                        }
                        glutin::VirtualKeyCode::W => {
                            self.state.up_is_pressed = true;
                        }
                        glutin::VirtualKeyCode::A => {
                            self.state.left_is_pressed = true;
                        }
                        glutin::VirtualKeyCode::S => {
                            self.state.down_is_pressed = true;
                        }
                        glutin::VirtualKeyCode::D => {
                            self.state.right_is_pressed = true;
                        }
                        glutin::VirtualKeyCode::Space => {
                            self.state.fire_is_pressed = true;
                        }
                        _ => {}
                    }
                }
                Event::Closed => {
                    self.state.exit = true;
                }
                _ => {}
            }
        }

        if self.gilrs.gamepads().count() != 0 {
            for _ in self.gilrs.poll_events() {}
            self.state.x_move = self.gilrs[0].value(gilrs::Axis::LeftStickX);
            self.state.y_move = self.gilrs[0].value(gilrs::Axis::LeftStickY);
            self.state.left_tilt = self.gilrs[0].value(gilrs::Axis::LeftZ);
            self.state.right_tilt = self.gilrs[0].value(gilrs::Axis::RightZ);
            self.state.fire_is_pressed = self.gilrs[0].is_pressed(gilrs::Button::South);
        } else {
            self.state.x_move = 0.0;
            if self.state.right_is_pressed {
                self.state.x_move += 0.5;
            }
            if self.state.left_is_pressed {
                self.state.x_move -= 0.5;
            }
            self.state.y_move = 0.0;
            if self.state.up_is_pressed {
                self.state.y_move += 0.5;
            }
            if self.state.down_is_pressed {
                self.state.y_move -= 0.5;
            }
            self.state.fire_is_pressed = self.state.fire_is_pressed;
        }
    }

    pub fn x_move(&self) -> AxisValue {
        self.state.x_move
    }

    pub fn y_move(&self) -> AxisValue {
        self.state.y_move
    }

    pub fn left_tilt(&self) -> AxisValue {
        self.state.left_tilt
    }

    pub fn right_tilt(&self) -> AxisValue {
        self.state.right_tilt
    }

    pub fn exit(&self) -> bool {
        self.state.exit
    }

    pub fn fire_is_pressed(&self) -> bool {
        self.state.fire_is_pressed
    }
}
