use macroquad::prelude::*;
use crate::miniquad::conf::Icon;

mod icon;
mod constants;
mod game;
mod grid;

use constants::{WINDOW_WIDTH, WINDOW_HEIGHT};
    
fn window_conf() -> Conf {
    let icon: Icon = icon::load_icon("data/icon.png");
    Conf {
        window_title: "Game of Life".to_string(),
        window_width: i32::try_from(WINDOW_WIDTH).expect("Window width too large for i32"),
        window_height: i32::try_from(WINDOW_HEIGHT).expect("Window height too large for i32"),
        fullscreen: false,
        icon: Some(icon),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = game::Game::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    game.game_loop().await;

}
