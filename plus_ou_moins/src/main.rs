mod game;
use crate::game::jeu::Jeu;
fn main() {
    let mut game = Jeu::new();
    game.run();
}
