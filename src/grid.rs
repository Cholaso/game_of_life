use crate::constants::{CELL_SIZE};
use macroquad::prelude::*;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cell_size: usize,
    cells: Vec<Vec<bool>>,
    last_update: f64,
}

impl Grid {
    // constructor
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cell_size: CELL_SIZE,
            cells: vec![vec![false; (width / CELL_SIZE)]; (height / CELL_SIZE)],
            last_update: 0.0,
        }
    }

    pub fn test(&mut self){
        let mut counter: usize = 0;
        for row in &mut self.cells {
            for cell in row {
                *cell = counter % 2 == 0;
                counter += 1;
            }
            counter += 1;
        }
    }

    pub fn update(&mut self) {

        let now = macroquad::time::get_time();

        if now - self.last_update < 1.0 {
            return;
        }

        self.last_update = now;

        for row in &mut self.cells {
            for cell in row.iter_mut() {
            *cell = !*cell;
            }
        }
    }


    pub fn draw(&self) {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let color: Color = if cell { BLACK } else { WHITE };
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

    fn get_neighbors(&self, index: usize) -> Vec<usize> {
        let mut neighbors: Vec<usize> = Vec::new();
        if index > 0 {
            neighbors.push(index - 1);
        }
        if index < self.width*self.height - 1 {
            neighbors.push(index + 1);
        }
        if index < self.width*self.height - self.width {
            neighbors.push(index + self.width);
        }
        if index >= self.width {
            neighbors.push(index - self.width);
        }
        neighbors
    }
}