mod gui;
mod game;
use crate::gui::gui::Gui;
fn main() {
    let mut app = Gui::new();
    app.run();
}
