use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod trail;
mod lorenz;
use trail::{Trail, create_default_trail};


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut window = video_subsystem
        .window("Lorenz Attractor Demo", 1200, 800)
        .position_centered()
        .build()
        .unwrap();
    window.set_fullscreen(sdl2::video::FullscreenType::True).expect("Expect Ok");

    let mut canvas = window.into_canvas().build().unwrap();
    let mut loops: i32 = 0;
    let mut trails: Vec<Trail> = vec![create_default_trail()];

    let scale: u32 = 12;
    canvas.set_scale(scale as f32, scale as f32).expect("Expect Ok");

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        if loops % 200 == 0 && loops <= 1200 {
            let new_trail:Trail = create_default_trail();
            trails.push(new_trail);
        }
        let mut new_trails:Vec<Trail> = Vec::new(); 
        
        for mut t in trails.clone() {
            t.draw(&mut canvas);
            new_trails.push(t);
        }

        trails = new_trails;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        loops += 1;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
