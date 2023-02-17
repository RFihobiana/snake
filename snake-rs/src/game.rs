use rand::random;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Coord(u16, u16);

struct Board {
    board_size: (u16, u16),

    snake: VecDeque<Coord>,
    food: Coord,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
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

    /// return the removed tail if no collision
    fn advance(&mut self, dir: Direction) -> Option<Coord> {
        let tail = self.snake.pop_back().unwrap();
        let next_head = move_cell(*self.snake.front().unwrap(), dir, self.board_size)?;
        let self_collision = self.snake.iter().find(|&&cell| cell == next_head).is_some();
        (!self_collision).then_some(tail)
    }

    /// return true iff no collision
    fn extend(&mut self, dir: Direction) -> bool {
        self.advance(dir)
            .and_then(|tail| Some(self.snake.push_back(tail)))
            .is_some()
    }

    fn move_food(&mut self) {
        let x: u16 = random();
        let y: u16 = random();
        let next = Coord(x, y);
        if self.snake.iter().find(|&&cell| cell == next).is_some() {
            self.move_food();
        } else {
            self.food = next;
        }
    }
}

fn move_cell_unchecked(cell: Coord, dir: Direction) -> Coord {
    match dir {
        Direction::Up => Coord(cell.0, cell.1 + 1),
        Direction::Down => Coord(cell.0, cell.1 - 1),
        Direction::Left => Coord(cell.0 - 1, cell.1),
        Direction::Right => Coord(cell.0 + 1, cell.1),
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
