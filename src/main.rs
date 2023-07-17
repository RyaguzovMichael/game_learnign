use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Duration;

struct GameState{
    game_tick : u8,
    colour : Color
}

fn main() -> Result<(), String> {
    let (mut event_pump, mut canvas) = init()?;

    let mut game_state = init_gamestate();

    loop {
        let result = process_input(&mut event_pump);
        if let Some(_) = result {
            break;
        }
        update(&mut game_state);
        render(&mut canvas, game_state.colour);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

fn init_gamestate() -> GameState {
    GameState{
        game_tick : 0,
        colour : Color::RGB(0, 255, 255)
    }
}

fn init() -> Result<(EventPump, Canvas<Window>), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let event_pump = sdl_context.event_pump()?;
    Ok((event_pump, init_canvas(init_window(video_subsystem))))
}

fn init_canvas(window: Window) -> Canvas<Window> {
    window
        .into_canvas()
        .build()
        .expect("could not make a canvas")
}

fn init_window(video_subsystem: sdl2::VideoSubsystem) -> sdl2::video::Window {
    video_subsystem
        .window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem")
}

fn process_input(event_pump: &mut EventPump) -> Option<bool> {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return Some(true);
            }
            _ => {
                return None;
            }
        }
    }
    return None;
}

fn update(game_state: &mut GameState) {
    game_state.game_tick = (game_state.game_tick + 1) % 255;
    game_state.colour = Color::RGB(game_state.game_tick, 64, 255 - game_state.game_tick);
}

fn render(canvas: &mut WindowCanvas, colour: Color) {
    canvas.set_draw_color(colour);
    canvas.clear();
    canvas.present();
}
