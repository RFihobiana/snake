use rand::random;
use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Coord(u16, u16);

pub struct Board {
    board_size: (u16, u16),

    snake: VecDeque<Coord>,
    food: Coord,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn invert(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Board {
    pub fn new(board_size: (u16, u16), len: u16) -> Self {
        let mut snake = VecDeque::from([Coord(0, 0)]);
        for i in 1..len {
            snake.push_front(Coord(i, 0));
        }
        let mut board = Board {
            board_size,
            snake,
            food: Coord(0, 1),
        };
        board.move_food();

        board
    }

    pub fn next(&mut self, dir: Direction) -> bool {
        let Some(prev_tail) = self.advance(dir) else {
            return false;
        };

        if self.snake.front().unwrap() == &self.food {
            self.snake.push_back(prev_tail);
            self.move_food();
        }

        return true;
    }

    /// return the removed tail if no collision
    fn advance(&mut self, dir: Direction) -> Option<Coord> {
        let tail = self.snake.pop_back().unwrap();
        let next_head = move_cell(*self.snake.front().unwrap(), dir, self.board_size)?;
        let self_collision = self.snake.iter().find(|&&cell| cell == next_head).is_some();
        self.snake.push_front(next_head);
        (!self_collision).then_some(tail)
    }

    // /// return true iff no collision
    // fn extend(&mut self, dir: Direction) -> bool {
    //     self.advance(dir)
    //         .and_then(|tail| Some(self.snake.push_back(tail)))
    //         .is_some()
    // }

    fn move_food(&mut self) {
        let x: u16 = random::<u16>() % self.board_size.0;
        let y: u16 = random::<u16>() % self.board_size.1;
        let next = Coord(x, y);
        if self.snake.iter().find(|&&cell| cell == next).is_some() {
            self.move_food();
        } else {
            self.food = next;
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, cell_size: (u32, u32)) {
        //
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // food
        canvas.set_draw_color(Color::RGB(120, 0, 0));
        canvas
            .fill_rect(self.get_rect(self.food, cell_size))
            .unwrap();

        // snake
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for &cell in self.snake.iter() {
            canvas.fill_rect(self.get_rect(cell, cell_size)).unwrap();
        }
    }

    fn get_rect(&self, cell: Coord, cell_size: (u32, u32)) -> Rect {
        Rect::new(
            (cell_size.0 * cell.0 as u32 + 1) as i32,
            ((self.board_size.1 - 1 - cell.1) as u32 * cell_size.1 + 1) as i32,
            cell_size.0 as u32 - 2,
            cell_size.1 as u32 - 2,
        )
    }
}

fn move_cell(cell: Coord, dir: Direction, board_size: (u16, u16)) -> Option<Coord> {
    match dir {
        Direction::Up => (cell.1 < board_size.1 - 1).then(|| Coord(cell.0, cell.1 + 1)),
        Direction::Down => (cell.1 > 0).then(|| Coord(cell.0, cell.1 - 1)),
        Direction::Left => (cell.0 > 0).then(|| Coord(cell.0 - 1, cell.1)),
        Direction::Right => (cell.0 < board_size.0 - 1).then(|| Coord(cell.0 + 1, cell.1)),
    }
}

pub struct Game {
    cell_size: (u32, u32),
    next_direction: Direction,
    current_direction: Direction,
    latest_move: Instant,
    speed: Duration,
    board: Board,
}

pub enum PollResult {
    None,
    Redraw,
    End,
}

impl Game {
    pub fn new(board_size: (u16, u16), cell_size: (u32, u32)) -> Game {
        Game {
            cell_size,
            next_direction: Direction::Right,
            current_direction: Direction::Right,
            latest_move: Instant::now(),
            speed: Duration::from_millis(600),
            board: Board::new(board_size, 3),
        }
    }

    pub fn set_direction(&mut self, dir: Direction) {
        if dir != self.current_direction.invert() {
            if dir == self.current_direction {
                self.latest_move = Instant::now().checked_sub(self.speed).unwrap();
            }
            self.next_direction = dir;
        }
    }

    pub fn poll(&mut self) -> PollResult {
        if self.latest_move.elapsed() > self.speed {
            self.latest_move = self.latest_move.checked_add(self.speed).unwrap();
            self.current_direction = self.next_direction;

            if !self.board.next(self.current_direction) {
                PollResult::End
            } else {
                PollResult::Redraw
            }
        } else {
            PollResult::None
        }
    }

    pub fn redraw(&self, canvas: &mut WindowCanvas) {
        self.board.draw(canvas, self.cell_size);
    }
}
