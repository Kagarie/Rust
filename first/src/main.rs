mod personne;

use personne::liste_personne::ListePersonne;

use std::time::Instant;


/// Calcul le temps d'éxécution
///```
///let mut list = ListePersonne::new();
///
/// /*Code ici*/
///
///print!("Time : {} secondes", now.elapsed().as_secs());
///```
fn main() {
    let mut list = ListePersonne::new();

    let now = Instant::now();
    list.random_personne(100);
    list.trie_age();
    list.to_string();

    //code here to see the time

    println!();

    print!("Time : {} secondes", now.elapsed().as_secs());

    println!();
}
