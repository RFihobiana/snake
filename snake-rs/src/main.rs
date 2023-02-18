mod game;

use game::{Board, Direction};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let cell_size = (40u16, 40u16);
    let board_size = (16u16, 16u16);

    let window = video_subsystem
        .window(
            "Snake",
            (cell_size.0 * board_size.0) as u32,
            (cell_size.1 * board_size.1) as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut board = Board::new((16, 16), 3);
    let start_time = Instant::now();

    let mut latest_move = start_time;
    let speed = Duration::from_millis(600);
    let mut direction = Direction::Right;
    let mut next_direction = Direction::Right;

    board.draw(&mut canvas, cell_size);

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
                } => next_direction = Direction::Up,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => next_direction = Direction::Down,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => next_direction = Direction::Left,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => next_direction = Direction::Right,
                _ => {}
            }
        }

        if latest_move.elapsed() > speed {
            if next_direction != direction.invert() {
                direction = next_direction;
            } else {
                next_direction = direction; // is this necessary?
            }
            latest_move = latest_move.checked_add(speed).unwrap();
            if !board.next(direction) {
                break 'running;
            }
        }

        board.draw(&mut canvas, cell_size);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
