use glium::backend::glutin_backend::WinRef;
use gilrs::Gilrs;

type AxisValue = f32;

struct InputState {
    x_move: AxisValue,
    y_move: AxisValue,
    fire_button: bool,
    exit: bool,
}

impl Default for InputState {
    fn default() -> InputState {
        InputState {
            x_move: 0.0,
            y_move: 0.0,
            fire_button: false,
            exit: false,
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
                Event::KeyboardInput(glutin::ElementState::Pressed,
                                     _,
                                     Some(glutin::VirtualKeyCode::Escape)) |
                Event::Closed => {
                    self.state.exit = true;
                }
                _ => {}
            }
        }

        for _ in self.gilrs.poll_events() {}

        self.state.x_move = self.gilrs[0].value(gilrs::Axis::LeftStickX);
        self.state.y_move = self.gilrs[0].value(gilrs::Axis::LeftStickY);
        self.state.fire_button = self.gilrs[0].is_pressed(gilrs::Button::South);
    }

    pub fn x_move(&self) -> AxisValue {
        self.state.x_move
    }

    pub fn y_move(&self) -> AxisValue {
        self.state.y_move
    }

    pub fn exit(&self) -> bool {
        self.state.exit
    }

    pub fn fire_button(&self) -> bool {
        self.state.fire_button
    }
}
