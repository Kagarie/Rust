//extern create rand;

use rand::Rng;
use std::io;
use std::str::FromStr;

fn recuperer_entree_utilisateur() -> Option<isize> {// elle ne prend rien en entrée et retourne un Option<isize> (dans le cas où ça ne fonctionnerait pas)
    let mut entree = String::new();

    match io::stdin().read_line(&mut entree) {// on récupère ce qu'a entré l'utilisateur dans la variable entree
        Ok(_) => {// tout s'est bien passé, on peut convertir la String en entier
            match isize::from_str(&entree.trim()) {
                Ok(nombre) => Some(nombre),// tout s'est bien déroulé, on retourne donc le nombre
                Err(_) => {
                    println!("Veuillez entrer un nombre valide !");
                    None
                }
            }
        },
        _ => {
            println!("Erreur lors de la récupération de la saisie...");
            None
        }
    }
}

fn jeu() -> bool {
    let mut tentative = 10;

    println!("Génération du nombre...");
    let nombre_aleatoire = rand::thread_rng().gen_range(1, 100);
    println!("C'est parti !");
    println!("Mon nombre est compis entre 1 et 100");
    while tentative > 0 {
        println!("Entrez un nombre : ");
        match recuperer_entree_utilisateur() {
            Some(nombre) => {
                if nombre < nombre_aleatoire {
                    println!("C'est plus grand !");
                } else if nombre > nombre_aleatoire {
                    println!("C'est plus petit !");
                } else {
                    return true;
                }
            }
            None => {}
        }
        tentative -= 1;
    }
    false
}

fn main() {
    println!("=== Jeu du plus ou moins ===");
    println!("");
    if jeu() == true {
        println!("Vous avez gagné !");
    } else {
        println!("Vous avez perdu...");
    }
}
