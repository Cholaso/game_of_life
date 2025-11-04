use macroquad::prelude::*;

use macroquad::input::{is_mouse_button_pressed, MouseButton};

use crate::grid::Grid;

pub struct Game {
    pub grid: Grid,
    paused: bool,

}
impl Game{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
            paused: false,
        }
    }

    pub async fn game_loop(&mut self) {
        self.grid.initialize_cells();
        // self.grid.test();
        loop {
            self.update();
            self.draw();
            next_frame().await;
        }
    }

    fn update(&mut self){
        self.handle_input();
        if !self.paused {
            self.grid.update();
        }
    }

    fn draw(&self) {
        clear_background(WHITE);
        for (y, row) in self.grid.cells().iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let color: Color = if cell.alive { BLACK } else { WHITE };
                draw_rectangle(
                    (x * self.grid.cell_size) as f32,
                    (y * self.grid.cell_size) as f32,
                    self.grid.cell_size as f32,
                    self.grid.cell_size as f32,
                    color,
                );
            }
        }
        if self.paused {
            draw_text("Paused", 10.0, 20.0, 30.0, RED);
        }
    }

    fn handle_input(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let grid_x = (mouse_x as usize) / self.grid.cell_size;
            let grid_y = (mouse_y as usize) / self.grid.cell_size;
            println!("Clicked on cell: ({}, {})", grid_x, grid_y);
            self.grid.update_cell_state(grid_x, grid_y);
        }
        if is_key_pressed(KeyCode::Space){
            self.paused = !self.paused;
            println!("Paused: {}", self.paused);
        }
    }
}
