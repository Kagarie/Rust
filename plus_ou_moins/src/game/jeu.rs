use crate::game::joueur::Joueur;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
#[derive(Clone)]
pub struct Jeu {
    nbr_hidden: i8,
    joueur: Joueur,
}

impl Jeu {
    pub fn new() -> Self {
        Self {
            nbr_hidden: 0,
            joueur: Joueur::new(),
        }
    }

    pub fn run(&mut self) {
        println!("Jeu du plus ou moins ! ");
        println!("Trouvé le nombre entre 0 et 100\n");

        self.nbr_hidden = rand::thread_rng().gen_range(1..101);

        loop {
            println!("Saisie : ");
            let mut number = String::new();
            io::stdin()
                .read_line(&mut number)
                .expect("Erreur de Lecture");
            self.joueur.set_input(match number.trim().parse() {
                Ok(number) => number,
                Err(_) => {
                    println!("Saisie incorrect");
                    continue;
                }
            });

            self.joueur.add_nbr_try();
            if self.verif() {
                self.joueur.add_score();
                if self.again() {
                    println!("Trouvé le nombre entre 0 et 100\n");
                    self.nbr_hidden = rand::thread_rng().gen_range(1..101);
                } else {
                    break;
                }
            }
        }
        self.end();
    }

    pub fn verif(&mut self) -> bool {
        match self.joueur.get_input().cmp(&self.nbr_hidden) {
            Ordering::Less => {
                println!("C'est plus !");
                return false;
            }
            Ordering::Greater => {
                println!("C'est moins !");
                return false;
            }
            Ordering::Equal => {
                println!("Nombre trouvé !\n");
                self.joueur
                    .add_liste(self.nbr_hidden, self.joueur.get_nbr_try());
                self.joueur.reset_try();
                return true;
            }
        }
    }

    pub fn again(&mut self) -> bool {
        let mut input = String::new();
        println!("\nNouvelle partie [Y/n]?");
        io::stdin()
            .read_line(&mut input)
            .expect("Erreur de lecture\n");
        input = input.trim().to_owned();
        if input.eq("Y") || input.eq("y") || input.eq("") {
            return true;
        } else {
            return false;
        }
    }

    pub fn end(&mut self) {
        let mut total_try: i8 = 0;
        for (key, value) in self.joueur.get_liste() {
            total_try += key;
            println!(
                "Nombre de coup joué {} pour trouver le nombre {}",
                key, value
            );
        }
        println!(
            "nombre de coup jouer : {} pour un total de {} victoire(s)\n",
            total_try,
            self.joueur.get_score()
        );
        println!("FIN DU JEU !\n");
    }
}
