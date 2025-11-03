use crate::constants::{CELL_SIZE};
use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub struct Cell {
    pub alive: bool,
    pub next_state: bool,
    pub x: usize,
    pub y: usize,
}

pub struct Grid {
    pub cell_size: usize,
    cells: Vec<Vec<Cell>>,
    last_update: f64,
}

impl Grid {
    // constructor
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cell_size: CELL_SIZE,
            cells: vec![vec![Cell { alive: false, next_state: false, x: 0, y: 0 }; width / CELL_SIZE]; height / CELL_SIZE],
            last_update: 0.0,
        }
    }

    pub fn initialize_cells(&mut self) {
        for (y, row) in self.cells.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                *cell = Cell {
                    alive: false,
                    next_state: false,
                    x: x,
                    y: y,
                };
            }
        }
    }

    pub fn test(&mut self){
        let mut counter: usize = 0;
        for row in &mut self.cells {
            for cell in row {
                *cell = Cell {
                    alive: counter % 2 == 0,
                    next_state: false,
                    x: cell.x,
                    y: cell.y,
                };
                counter += 1;
            }
            counter += 1;
        }
    }

    pub fn update(&mut self) {

        let now = macroquad::time::get_time();

        if now - self.last_update < 0.75 {
            return;
        }

        self.last_update = now;

        let old_cells = self.cells.clone(); // clone the grid for reading
        for (y, row) in self.cells.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                let mut live_neighbors = 0;

                for ny in y.saturating_sub(1)..=(y + 1).min(old_cells.len() - 1) {
                    for nx in x.saturating_sub(1)..=(x + 1).min(old_cells[0].len() - 1) {
                        if (nx != x || ny != y) && old_cells[ny][nx].alive {
                            live_neighbors += 1;
                        }
                    }
                }

                cell.next_state = match (cell.alive, live_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }

        for row in &mut self.cells {
            for cell in row.iter_mut() {
                cell.alive = cell.next_state;
            }
        }
    }


    pub fn draw(&self) {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let color: Color = if cell.alive { BLACK } else { WHITE };
                draw_rectangle(
                    (x * self.cell_size) as f32,
                    (y * self.cell_size) as f32,
                    self.cell_size as f32,
                    self.cell_size as f32,
                    color,
                );
            }
        }
    }
}