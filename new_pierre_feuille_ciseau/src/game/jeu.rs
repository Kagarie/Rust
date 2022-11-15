use std::{collections::HashMap, io};

use crate::game::joueur::Joueur;

#[derive(Clone)]
pub(crate) struct Jeu {
    joueur: Joueur,
    ordinateur: Joueur,
    jeu: HashMap<i8, String>,
}

impl Jeu {

    pub fn new() -> Self {
        let ordi = String::from("Ordinateur");
        let mut joueur = String::new();
        io::stdin()
            .read_line(&mut joueur)
            .expect("Erreur de lecture");
        Self {
            joueur: Joueur::new(joueur.trim().to_owned()),
            ordinateur: Joueur::new(ordi),
            jeu: HashMap::from([
                (1, String::from("Pierre")),
                (2, String::from("Feuille")),
                (3, String::from("Ciseaux")),
            ]),
        }
    }

    pub fn run(&mut self) {
        let mut run = true;
        while run {
            
            run = self.new_partie();
        }
        self.victoire();
    }

    pub fn new_partie(&mut self) -> bool {
        let mut input = String::new();
        println!("Nouvelle partie [Y/n]?");
        io::stdin()
            .read_line(&mut input)
            .expect("Erreur de lecture");
        input = input.trim().to_owned();
        if input.eq("Y") || input.eq("y") || input.eq("") {
            self.joueur.reset_point();
            return true;
        } else {
            return false;
        }
    }

    pub fn victoire(&mut self)  {
        if self.joueur.get_victoire() > self.ordinateur.get_victoire() {
            println!("Victoire du joueur {} \n", self.joueur.get_name());
        } else {
            println!("Victoire de l'ordinateur");
        }
        println!(
            "Résultat:\nOrdinateur : {}\n{} : {}",
            self.ordinateur.get_victoire(),
            self.joueur.get_name(),
            self.joueur.get_point(),
        );
        println!("\nFin du jeu");
    }
}
