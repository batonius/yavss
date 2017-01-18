extern crate gilrs;
#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

mod input;
mod sprites_data;
mod scene;
mod display;

use glium::glutin;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const VIRTUAL_WIDHT: u32 = 128;
const VIRTUAL_HEIGHT: u32 = 128;
const FRAME_RATE: u64 = 60;

fn create_window() -> glium::backend::glutin_backend::GlutinFacade {
    use glium::DisplayBuild;
    glutin::WindowBuilder::new()
        .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_gl(glutin::GlRequest::Latest)
        .with_gl_profile(glutin::GlProfile::Core)
        .with_vsync()
        .with_title("gliumpad")
        .build_glium()
        .expect("Can't create gluim window")
}

fn main() {
    use std::time::Instant;

    let frame_rate_loop_duration = Duration::from_millis(1_000u64 / FRAME_RATE);
    let mut scene = scene::Scene::new(scene::SpeedValues::new(0.75, 0.75, 0.15, 2.0));
    let window = create_window();
    let mut input_poller = input::InputPoller::new(window.get_window()
        .expect("Can't get window ref"));
    let mut instant = Instant::now();
    let sprites = sprites_data::SpritesData::new((VIRTUAL_WIDHT, VIRTUAL_HEIGHT));
    let renderer = display::Renderer::new(&window, &sprites);

    'main_loop: loop {
        let mut new_instant = Instant::now();
        let mut duration = new_instant - instant;
        if duration < frame_rate_loop_duration {
            std::thread::sleep(frame_rate_loop_duration - duration);
            new_instant = Instant::now();
            duration = new_instant - instant;
        }
        instant = new_instant;
        input_poller.poll_events();
        if input_poller.exit() {
            break 'main_loop;
        }
        scene.tick(&input_poller, duration);
        renderer.render(&window, &scene);
    }
}
