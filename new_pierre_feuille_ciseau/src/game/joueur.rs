#[derive(Clone)]
pub struct Joueur {
    name: String,
    point: i8,
    nbr: i8,
}

impl Joueur {
    ///Contructeur
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            point: 0,
            nbr: 0,
        }
    }

    ///Getteurs

    pub fn get_name(&self) -> &str {
        return &self.name;
    }
    pub fn get_point(&self) -> i8 {
        return self.point;
    }
    pub fn get_nbr(&self) -> i8 {
        return self.nbr;
    }
    //Setteur
    pub fn set_nbr(&mut self, nbr: i8) {
        self.nbr = nbr
    }

    ///Incr point
    pub fn add_point(&mut self) {
        self.point += 1;
    }
}
