extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::image::InitFlag;
use sdl2::image::LoadTexture;

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsys = sdl_ctx.video().unwrap();

    let window = video_subsys
        .window("Hello SDL", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .unwrap();

    sdl2::image::init(InitFlag::PNG).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("./assets/hello_sdl.png").unwrap();

    let mut event_pump = sdl_ctx.event_pump().unwrap();
    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
    }
}
