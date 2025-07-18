use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::Texture;
use sdl2::surface::Surface;
use std::time::Duration;


pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 100)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    window.set_opacity(0.8)?;
    // if let Err(err) = window.set_opacity(230.0) {
    //     panic!("error:{:?}", err);
    // }

    // let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().unwrap();
    // let mut canvas = window.into_canvas().build()?;

    canvas.set_draw_color(Color::RGBA(255, 233, 0, 33));
    let texture_creator = canvas.texture_creator();
    let surface = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
    let texture = Texture::from_surface(&surface, &texture_creator).unwrap();

    canvas.clear();
    canvas.present();
    // println!("{:?}", canvas);
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::TextInput { text, .. } => {
                    println!("xxx:{}", text);
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // canvas.set_draw_color(Color::RGB(55, 233, 88));
        // canvas.set_draw_color(Color::RGBA(255, 233, 0, 0));

        // println!("aa");
        canvas.clear();
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
