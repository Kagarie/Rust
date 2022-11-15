use rand::Rng;
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
        println!("Nom du joueur : ");
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
        let mut input = String::new();
        let mut ordi: i8 = 0;
        while run {
            ordi = rand::thread_rng().gen_range(1..3);
            println!("Pierre [1],Feuille [2] ,Ciseaux[3]\nSaisie :");
            io::stdin()
                .read_line(&mut input)
                .expect("Erreur de lecture");

            let number: i8 = match input.trim().parse() {
                Ok(number) => {
                    if number > 3 || number < 1 {
                        println!("Le nombre saisie doit être compris entre 1 et 3 !");
                        continue;
                    } else {
                        number
                    }
                }
                Err(_) => {
                    println!("Erreur");
                    continue;
                }
            };

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

    pub fn victoire(&mut self) {
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

    pub fn verif_jeu(&mut self, nbr_ordi: i8, nbr_joueur: i8) {
        if nbr_joueur == nbr_ordi {
            println!("Manche nul");
        } else if nbr_ordi == 1 && nbr_joueur == 3
            || nbr_ordi == 2 && nbr_joueur == 1
            || nbr_ordi == 3 && nbr_joueur == 2
        {
            println!("Manche perdu ! ");
            print!(
                "ordinateur : {}\n{},{}",
                self.jeu[&nbr_ordi],
                self.joueur.get_name(),
                self.jeu[&nbr_joueur]
            );
            self.ordinateur.add_point();
        } else if nbr_joueur == 1 && nbr_ordi == 3
            || nbr_joueur == 2 && nbr_ordi == 1
            || nbr_joueur == 3 && nbr_ordi == 2
        {
            println!("Manche gagné ! ");
            print!(
                "ordinateur : {}\n{},{}",
                self.jeu[&nbr_ordi],
                self.joueur.get_name(),
                self.jeu[&nbr_joueur]
            );
            self.joueur.add_point();
        }
        println!();
        println!(
            "{} a {} points",
            self.joueur.get_name(),
            self.joueur.get_point()
        );
        println!("L'ordi a {} points", self.ordinateur.get_point());
        println!(
            "A {} a {} points",
            self.joueur.get_name(),
            self.joueur.get_point()
        );
    }
}
