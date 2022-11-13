mod personne;

use personne::liste_personne::ListePersonne;
use personne::personne::Personne;

use std::time::Instant;

fn main() {
    let mut list = ListePersonne::new();

    let now = Instant::now();

    //code here to see the time 

    println!();

    print!("Time : {} secondes", now.elapsed().as_secs());

    println!();
}
