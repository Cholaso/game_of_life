use macroquad::prelude::*;

use crate::grid::Grid;

pub struct Game {
    pub grid: Grid,

}
impl Game{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
        }
    }

    pub async fn game_loop(&mut self) {
        self.grid.initialize_cells();
        self.grid.test();
        loop {
            self.grid.update();
            clear_background(WHITE);
            self.grid.draw();
            next_frame().await;
        }
    }
}
