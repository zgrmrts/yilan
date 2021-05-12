mod canvas;
mod direction;
mod element;
mod game;
mod point;

use game::Game;

pub fn main() {
    let mut game = Game::new();
    game.main_loop();
}
