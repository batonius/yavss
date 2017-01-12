extern crate gilrs;
#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

mod input;
mod scene;
mod display;

use glium::glutin;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
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
    let mut scene = scene::Scene::new(scene::SpeedValues::new(0.25, 0.25, 0.06));
    let window = create_window();
    let mut input_poller = input::InputPoller::new(window.get_window()
        .expect("Can't get window ref"));
    let mut instant = Instant::now();
    let renderer = display::Renderer::new(&window);
    let mut counter = 0;

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
        let mut target = window.draw();
        renderer.render(&mut target, &scene);
        target.finish().expect("Can't draw on a surface");
        counter += 1;
        if counter % 60 == 0 {
            println!("{:#?}", scene);
        }
    }
}
