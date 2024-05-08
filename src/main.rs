extern crate piston_window;

// modules

use piston_window::*;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::time::Instant;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Snake Game", [640, 640])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_ups(60);
    let mut snake = Snake::new(10, 10);
    let mut board = GameBoard::new(20, 20);
    board.place_food();
    board.recursive_place_food();

    while let Some(event) = window.next() {
        if let Some(update_args) = event.update_args() {
            println!("Updating Game...");
            if let Some(&(x, y)) = snake.body.get(0) {
                // Safely access and update the grid
                if let Some(row) = board.grid.get_mut(x) {
                    if let Some(cell) = row.get_mut(y) {
                        *cell = Cell::Snake;
                    } else {
                        println!("Invalid Y index in grid.");
                    }
                } else {
                    println!("Invalid X index in grid.");
                }
            } else {
                println!("No snake body found at index 0.");
            }
        }
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0, 1.0, 1.0, 1.0], graphics);
            for r in 0..board.height {
                for c in 0..board.width {
                    let x = c as f64 * 32.0;
                    let y = r as f64 * 32.0;
                    let color = match board.grid[r][c] {
                        Cell::Food => [0.8, 0.0, 0.0, 1.0], // Red color for food
                        Cell::Snake => [0.0, 0.8, 0.0, 1.0], // Green color for snake
                        Cell::Empty => continue, // Nothing color for empty.
                    };
                    rectangle(color, [x, y, 8.0, 8.0], context.transform, graphics);
                }
            }
        });
    }
}

struct GameBoard {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
}

impl GameBoard {
    pub fn new(width: usize, height: usize) -> GameBoard {
        let grid = vec![vec![Cell::Empty; width]; height];
        GameBoard { width, height, grid }
    }

    pub fn reset(&mut self) {
        for row in self.grid.iter_mut() {
            for cell in row.iter_mut() {
                *cell = Cell::Empty;
            }
        }
    }

    pub fn place_food(&mut self) {
        let start = Instant::now();
        let mut rng = thread_rng();
        loop {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            if self.grid[y][x] == Cell::Empty && self.grid[y][x] != Cell::Snake {
                self.grid[y][x] = Cell::Food;
                break;
            }
        }
        let duration = start.elapsed();
        println!("Iterative Time taken: {:?}", duration);
        return;
    }
    pub fn recursive_place_food(&mut self) {
        let start = Instant::now();
        let mut rng: ThreadRng = thread_rng();
        let x: usize = rng.gen_range(0..self.width);
        let y: usize = rng.gen_range(0..self.height);
        if self.grid[y][x] != Cell::Empty {
            self.recursive_place_food();
        } else {
            self.grid[y][x] = Cell::Food;
        }
        let duration = start.elapsed();
        println!("Recursive Time taken: {:?}", duration);
        return;
    }
}

struct Snake {
    body: Vec<(usize, usize)>,
    direction: Direction,
    size: u32,
}

impl Snake {
    fn new(start_x: usize, start_y: usize) -> Snake {
        let initial_body = vec![(start_x, start_y)];
        let initial_size: u32 = 5;
        Snake {
            body: initial_body,
            direction: Direction::Right,
            size: initial_size,
        }
    }
    fn move_forward(&mut self) {
        let (head_x, head_y) = self.body[0];
        let new_head = match self.direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        };
        self.body.insert(0, new_head);
        self.body.pop();
    }
    fn change_direction(&mut self, new_direction: Direction) {
        if new_direction.opposite() != self.direction {
            self.direction = new_direction;
        }
    }
    fn grow(&mut self) {
        let last_segment = *self.body.last().unwrap();
        self.body.push(last_segment);
    }
}

impl Direction {
    fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Cell {
    Empty,
    Snake,
    Food,
}
