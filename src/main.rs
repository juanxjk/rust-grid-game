mod game;

use game::Game;

fn main() {
    println!("Starting game...");
    let mut game = Game::new();
    game.start();
}
