#[derive(Clone)]
pub struct Joueur {
    nbr_try: i8,
    score: i8,
    input: i8,
    liste: Vec<(i8,i8)>,
}

impl Joueur {
    pub fn new() -> Self {
        Self {
            nbr_try: 0,
            score: 0,
            input: 0,
            liste: Vec::new(),
        }
    }

    pub fn get_nbr_try(&self) -> i8 {
        return self.nbr_try;
    }
    pub fn get_score(&self) -> i8 {
        return self.score;
    }
    pub fn get_input(&self) -> i8 {
        return self.input;
    }
    pub fn get_liste(&self) -> Vec<(i8, i8)> {
        return self.liste.clone();
    }
    pub fn set_input(&mut self, input: i8) {
        self.input = input;
    }
    pub fn add_nbr_try(&mut self) {
        self.nbr_try += 1;
    }
    pub fn reset_try(&mut self){
        self.nbr_try = 0 ;
    }

    pub fn add_score(&mut self) {
        self.score += 1;
    }
    pub fn add_liste(&mut self, nbr_hidden: i8, nbr_coup: i8) {
        self.liste.push((nbr_coup, nbr_hidden));
    }
}
