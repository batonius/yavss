use glium::backend::glutin_backend::WinRef;
use gilrs::Gilrs;

type AxisValue = f32;

struct InputStatus {
    x_move: AxisValue,
    y_move: AxisValue,
    fire_button: bool,
    exit: bool,
}

impl Default for InputStatus {
    fn default() -> InputStatus {
        InputStatus {
            x_move: 0.0,
            y_move: 0.0,
            fire_button: false,
            exit: false,
        }
    }
}

impl InputStatus {
    fn clear(&mut self) {
        *self = Default::default();
    }
}

pub struct InputPoller<'a> {
    status: InputStatus,
    gilrs: Gilrs,
    win: WinRef<'a>,
}

impl<'a> InputPoller<'a> {
    pub fn new(win: WinRef<'a>) -> InputPoller<'a> {
        InputPoller {
            status: Default::default(),
            gilrs: Gilrs::new(),
            win: win,
        }
    }

    pub fn poll_events(&mut self) {
        use glium::glutin;
        use glium::glutin::Event;
        use gilrs;

        self.status.clear();

        for event in self.win.poll_events() {
            match event {
                Event::KeyboardInput(glutin::ElementState::Pressed,
                                     _,
                                     Some(glutin::VirtualKeyCode::Escape)) |
                Event::Closed => {
                    self.status.exit = true;
                }
                _ => {}
            }
        }

        for _ in self.gilrs.poll_events() {}

        self.status.x_move = self.gilrs[0].value(gilrs::Axis::LeftStickX);
        self.status.y_move = self.gilrs[0].value(gilrs::Axis::LeftStickY);
        self.status.fire_button = self.gilrs[0].is_pressed(gilrs::Button::South);
    }

    pub fn x_move(&self) -> AxisValue {
        self.status.x_move
    }

    pub fn y_move(&self) -> AxisValue {
        self.status.y_move
    }

    pub fn exit(&self) -> bool {
        self.status.exit
    }

    pub fn fire_button(&self) -> bool {
        self.status.fire_button
    }
}
