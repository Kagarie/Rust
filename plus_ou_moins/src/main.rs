use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Jeu du plus ou moins ! ");
    println!("Trouvé le nombre entre 0 et 100");

    // Génération d'un nombre entre 0 et 100
    let _res = rand::thread_rng().gen_range(1..101);

    //Compteur du nombre de saisie de l'utilisateur
    let mut number_saisie = 0;

    //Boucle du jeu
    loop {
        println!("Saisie : ");
        let mut number = String::new();

        // On récupére la saisie de l'utilisateur et incrementation du compteur
        io::stdin().read_line(&mut number).expect("Error reading");
        number_saisie+=1;

        /*
        On vérifie si la saisie de l'utilisateur peut être parsé en nombre
        Sinon on gère le cas d'erreur
         */
        let number: u32 = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Saisie incorrect");
                continue
            },
        };

        //On compare les valeurs et on arrête le programme si la valeurs et trouvé
        match number.cmp(&_res) {
            Ordering::Less =>println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal =>{
                println!("Vous avez gagné !");
                println!("Nombre de saisie {}",number_saisie);
                break;}
        }
    }
}
