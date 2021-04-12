extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use std::time::{Duration, Instant};
mod loopy;
use loopy::{Game,GameObject};


pub fn main() -> Result<(), String> {
    let mut game = Game::new();
    game.set_assets_root("./assets/");
    game.set_draw_background(true);

    let mut obj1 = GameObject::init(game);
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    // Load a font
    let font = ttf_context.load_font(std::path::Path::new("src/Roboto.ttf"), 20)?;
    let texture_creator = canvas.texture_creator();

    let mut start = Instant::now();
    'running: loop {
        let delta_time = start.elapsed();
        start = Instant::now();
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

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let mut fps = 0;
        if delta_time.as_millis() > 0{
            fps = 1000 / delta_time.as_millis();
        }
        let surface = font
            .render(&(format!("FPS: {}", fps))[..])
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        // If the example text is too big for the screen, downscale it (and center irregardless)
        let target = Rect::new(10, 10, width, height);

        canvas.copy(&texture, None, Some(target))?;
        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
        // The rest of the game loop goes here...

    }

    Ok(())
}
