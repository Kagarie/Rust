#[derive(Clone)]
pub struct Joueur {
    choix: i8,
    list_coup: Vec<(String, bool)>,
}

impl Joueur {
    pub fn new() -> Self {
        Self {
            choix: 0,
            list_coup: Vec::new(),
        }
    }

    pub fn get_choix(&self) -> i8 {
        return self.choix;
    }

    pub fn get_liste_coup(&self) -> Vec<(String, bool)> {
        return self.list_coup.clone();
    }
    pub fn set_choix(&mut self, choix: i8) {
        self.choix = choix;
    }

    pub fn add_coup(&mut self, name_coup: String, res: bool) {
        self.list_coup.push((name_coup, res));
    }

    pub fn nbr_reussite(&mut self) -> i8 {
        let mut res: i8 = 0;
        for (_string, _bool) in self.list_coup.iter() {
            if *_bool {
                res += 1;
            }
        }
        return res;
    }

    pub fn nbr_echec(&self) -> i8 {
        let mut res: i8 = 0;
        for (_string, _bool) in self.list_coup.iter() {
            if !*_bool {
                res += 1;
            }
        }
        return res;
    }
}
