use rand::Rng;
use std::cmp::Ordering;
#[derive(Clone, Copy)]
pub struct Jeu {
    nbr_hidden: i8,
    nbr_coup: i8,
    nbr_victoire: i8,
    nbr_coup_total: i8,
}

impl Jeu {
    pub fn new() -> Self {
        Self {
            nbr_hidden: rand::thread_rng().gen_range(1..101),
            nbr_coup: 0,
            nbr_victoire: 0,
            nbr_coup_total: 0,
        }
    }

    pub fn get_nbr_coup(&self) -> i8 {
        return self.nbr_coup;
    }
    pub fn get_nbr_victoire(&self) -> i8 {
        return self.nbr_victoire;
    }
    pub fn get_nbr_coup_total(&self) -> i8 {
        return self.nbr_coup_total;
    }
    pub fn add_coup(&mut self) {
        self.nbr_coup += 1;
    }
    pub fn add_victoire(&mut self) {
        self.nbr_victoire += 1;
    }
    pub fn calcul_total(&mut self) {
        self.nbr_coup_total += self.nbr_coup;
    }
    pub fn launch(&mut self) {
        self.nbr_hidden = rand::thread_rng().gen_range(1..101);
        self.nbr_coup = 0;
    }

    pub fn verif(&self, input: String) -> (&str, i8) {
        let input_parse: i8 = input.trim().parse().unwrap();
        match input_parse.cmp(&self.nbr_hidden) {
            Ordering::Less => return ("C'est plus !", 0),
            Ordering::Greater => return ("C'est moins !", 0),
            Ordering::Equal => return ("Nombre trouvé !", 1),
        }
    }
}
