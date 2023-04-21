use crate::game::jeu::{self, Jeu};
use fltk::{
    app::{self, App},
    button::Button,
    frame::Frame,
    prelude::*,
    window::Window,
    *,
};
pub struct Gui {
    app: App,
    wind: Window,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            app: app::App::default(),
            wind: Window::new(400, 400, 400, 300, "Nombre secret"),
        }
    }

    pub fn run(&mut self) {
        let mut game = Jeu::new();
        let mut victoire = false;

        let frame = Frame::default()
            .with_size(self.wind.width(), 20)
            .with_label("Nombre compris entre 0 et 100 : ");

        let input = input::IntInput::default()
            .with_pos(self.wind.width() / 4, &frame.y() + &frame.height() + 20)
            .with_size(150, 30);

        let mut but_submit = Button::default()
            .with_pos(self.wind.width() / 4 * 3, &frame.y() + &frame.height() + 20)
            .with_size(60, 25)
            .with_label("Valider");

        let mut frame_message = Frame::default()
            .with_pos(0, &input.y() + &input.height() + 20)
            .with_size(self.wind.width() / 2, 20);

        let mut frame_nbr_coup = Frame::default()
            .with_pos(0, &frame_message.y() + &frame_message.height() + 20)
            .with_size(self.wind.width() / 2, 20)
            .with_label("Nombre de coup joué :0");

        let mut frame_nbr_victoire = Frame::default()
            .with_pos(0, &frame_nbr_coup.y() + &frame_nbr_coup.height() + 20)
            .with_size(self.wind.width() / 2, 20)
            .with_label("Nombre de victoire\t:0");

        let mut but_again = Button::default()
            .with_pos(
                self.wind.width() / 3,
                &frame_nbr_victoire.y() + &frame_nbr_victoire.height() + 20,
            )
            .with_size(150, 25)
            .with_label("Nouvelle partie");

        self.wind.make_resizable(true);
        self.wind.end();
        self.wind.show();
        self.app.run().unwrap();    

        but_submit.set_callback(move |_| {
            game.add_coup();
            let mut tmp_game = game;

            let res = game.verif(input.value());
            if res.1 == 1 {
                tmp_game.add_victoire();
                tmp_game.calcul_total();
                victoire = true;
            }

            frame_message.set_label(res.0);

            let nbr_coup: &str =
                &("Nombre de coup joué :".to_owned() + &tmp_game.get_nbr_coup().to_string());
            let nbr_victoire: &str =
                &("Nombre de victoire\t:".to_owned() + &tmp_game.get_nbr_victoire().to_string());
            frame_nbr_coup.set_label(nbr_coup);
            frame_nbr_victoire.set_label(nbr_victoire);

            game = tmp_game;
        });
        if victoire {
            but_submit.hide();
            but_again.show();
        } else {
            but_submit.show();
            but_again.hide();
        }
    }

    fn call_submit(){
        
    }
}
