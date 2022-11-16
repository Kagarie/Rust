use rand::Rng;
use std::{collections::HashMap, io};

use crate::game::joueur::Joueur;

#[derive(Clone)]
pub(crate) struct Jeu {
    joueur: Joueur,
    ordinateur: Joueur,
    jeu: HashMap<i8, String>,
}
pub const MESSAGE_ERREUR: &str = "Erreur de lecture";
impl Jeu {
    ///Constructeur
    pub fn new() -> Self {
        let ordi = String::from("Ordinateur");
        let mut joueur = String::new();
        println!("Nom du joueur : ");
        io::stdin().read_line(&mut joueur).expect(MESSAGE_ERREUR);
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

    ///Boucle du jeu
    pub fn run(&mut self) {
        let mut run = true;
        while run {
            let mut input = String::new();
            self.ordinateur.set_nbr(rand::thread_rng().gen_range(1..3));
            println!("\nPierre [1],Feuille [2] ,Ciseaux[3]\nSaisie :");
            io::stdin().read_line(&mut input).expect(MESSAGE_ERREUR);

            //Parse de la saisie en nombre et on verifie les conditions de saisie
            self.joueur.set_nbr(match input.trim().parse() {
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
            });
            self.verif_jeu();
            run = self.new_partie();
        }
        self.victoire();
    }

    pub fn new_partie(&mut self) -> bool {
        let mut input = String::new();
        println!("\nNouvelle partie [Y/n]?");
        io::stdin()
            .read_line(&mut input)
            .expect("Erreur de lecture");
        input = input.trim().to_owned();
        if input.eq("Y") || input.eq("y") || input.eq("") {
            return true;
        } else {
            return false;
        }
    }

    ///Verification des coups joué et attribution des points
    pub fn verif_jeu(&mut self) {
        if self.joueur.get_nbr() == self.ordinateur.get_nbr() {
            println!("\nManche nul");
        } else if self.ordinateur.get_nbr() == 1 && self.joueur.get_nbr() == 3
            || self.ordinateur.get_nbr() == 2 && self.joueur.get_nbr() == 1
            || self.ordinateur.get_nbr() == 3 && self.joueur.get_nbr() == 2
        {
            println!("\nManche perdu !");
            print!(
                "ordinateur : {}\n{} : {}\n",
                self.jeu[&self.ordinateur.get_nbr()],
                self.joueur.get_name(),
                self.jeu[&self.joueur.get_nbr()]
            );
            self.ordinateur.add_point();
        } else if self.joueur.get_nbr() == 1 && self.ordinateur.get_nbr() == 3
            || self.joueur.get_nbr() == 2 && self.ordinateur.get_nbr() == 1
            || self.joueur.get_nbr() == 3 && self.ordinateur.get_nbr() == 2
        {
            println!("\nManche gagné !");
            print!(
                "ordinateur : {}\n{} : {}\n",
                self.jeu[&self.ordinateur.get_nbr()],
                self.joueur.get_name(),
                self.jeu[&self.joueur.get_nbr()]
            );
            self.joueur.add_point();
        }
        println!(
            "\n{} a {} points",
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

    ///Fin de partie , affichage du gagné et des scores
    pub fn victoire(&mut self) {
        if self.joueur.get_point() < self.ordinateur.get_point() {
            println!("Victoire du joueur {} \n", self.joueur.get_name());
        } else if self.joueur.get_point() == self.ordinateur.get_point() {
            println!("\nPartie null\n");
        } else {
            println!("\nVictoire de l'ordinateur\n");
        }
        println!(
            "Résultat:\nOrdinateur : {}\n{} : {}",
            self.ordinateur.get_point(),
            self.joueur.get_name(),
            self.joueur.get_point(),
        );
        println!("\nFIN DU JEU\n");
    }
}
