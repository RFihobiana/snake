mod game;

use game::{Direction, Game, PollResult};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let cell_size = (40u32, 40u32);
    let board_size = (16u16, 16u16);

    let window = video_subsystem
        .window(
            "Snake",
            cell_size.0 * board_size.0 as u32,
            cell_size.1 * board_size.1 as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game = Game::new(board_size, cell_size);

    game.redraw(&mut canvas);
    canvas.present();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => game.set_direction(Direction::Up),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => game.set_direction(Direction::Down),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => game.set_direction(Direction::Left),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => game.set_direction(Direction::Right),
                _ => {}
            }
        }

        match game.poll() {
            PollResult::Redraw => game.redraw(&mut canvas),
            PollResult::End => break 'running,
            PollResult::None => (),
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
