use rand::Rng;
use std::{collections::HashMap, io};

use super::joueur::Joueur;

#[derive(Clone)]
pub struct Jeu {
    joueur: Joueur,
    ordi: i8,
    liste_choix: HashMap<i8, String>,
}
const MESSAGE_ERREUR: &str = "Erreur de lecture";
impl Jeu {
    pub fn new() -> Self {
        Self {
            joueur: Joueur::new(),
            ordi: 0,
            liste_choix: HashMap::from([(1, String::from("Pile")), (2, String::from("Face"))]),
        }
    }

    //Boucle principal
    pub fn run(&mut self) {
        let mut res :bool;
        loop {
            self.ordi = rand::thread_rng().gen_range(1..3);

            let mut input = String::new();
            println!("Pile ou Face [1/2]?");
            io::stdin().read_line(&mut input).expect(MESSAGE_ERREUR);
            self.joueur.set_choix(match input.trim().parse() {
                Ok(input) => {
                    if input != 1 && input != 2 {
                        println!("Le nombre saisie doit être 1 ou 2 ");
                        continue;
                    } else {
                        input
                    }
                }
                Err(_) => {
                    println!("Erreur");
                    continue;
                }
            });
            if self.ordi == self.joueur.get_choix() {
                res = true;
                println!("Bonne réponse");
            } else {
                res = false;
                println!("Mauvaise réponse");
            }
            self.joueur
                .add_coup(self.liste_choix[&self.joueur.get_choix()].clone(), res);
            if !self.new_partie() {
                break;
            }
        }
        self.end_game();
    }

    pub fn new_partie(&mut self) -> bool {
        let mut input = String::new();
        println!("\nNouvelle partie [Y/n]?");
        io::stdin().read_line(&mut input).expect(MESSAGE_ERREUR);
        input = input.trim().to_owned();
        if input.eq("Y") || input.eq("y") || input.eq("") {
            return true;
        } else {
            return false;
        }
    }

    pub fn end_game(&mut self) {
        println!("\nListe de vos coups joué : ");
        for (coup, res) in self.joueur.get_liste_coup().iter() {
            println!("  - {} , resultat : {}", coup, res);
        }
        println!(
            "\nVous avez gagné {} fois et perdu {} fois.\nPour un total de {} coups.",
            self.joueur.nbr_reussite(),
            self.joueur.nbr_echec(),
            self.joueur.get_liste_coup().len()
        );
        println!("\nFIN DU JEU\n");
    }
}
