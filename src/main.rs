extern crate gilrs;
#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

mod util;
mod input;
mod sprites;
mod collision;
mod scene;
mod display;

use glium::glutin;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const VIRTUAL_WIDHT: u32 = 200;
const VIRTUAL_HEIGHT: u32 = 200;
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

    let virtual_dimensions = (VIRTUAL_WIDHT, VIRTUAL_HEIGHT);
    let frame_rate_loop_duration = Duration::from_millis(1_000u64 / FRAME_RATE);
    let sprites = sprites::SpritesData::new(virtual_dimensions);
    println!("{:?}", sprites.sprite_data(sprites::SpriteObject::Player));
    println!("{:?}",
             sprites.sprite_data(sprites::SpriteObject::EnemyBullet));
    println!("{:?}",
             sprites.sprite_data(sprites::SpriteObject::PlayerBullet));
    let mut scene = scene::Scene::new(&sprites);
    let window = create_window();
    let mut input_poller = input::InputPoller::new(window.get_window()
        .expect("Can't get window ref"));
    let mut instant = Instant::now();
    let mut renderer = display::Renderer::new(&window, &sprites, virtual_dimensions);
    let mut frame_counter = 0usize;
    let mut frame_counter_instant = Instant::now();
    const FRAMES_TO_COUNT: usize = 600;

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

        frame_counter += 1;
        if frame_counter >= FRAMES_TO_COUNT {
            let new_frame_instant = Instant::now();
            let duration = new_frame_instant - frame_counter_instant;
            println!("{} fps",
                     frame_counter as f32 /
                     (duration.as_secs() as f32 +
                      duration.subsec_nanos() as f32 / 1_000_000_000.0));
            frame_counter = 0;
            frame_counter_instant = Instant::now();
        }
    }
}
