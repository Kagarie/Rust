mod game;
use crate::game::jeu::Jeu;
use std::process::exit;

fn main() {
    let mut game = Jeu::new();
    game.run();
    exit(0)
}
