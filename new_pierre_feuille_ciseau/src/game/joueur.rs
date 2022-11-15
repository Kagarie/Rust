#[derive(Clone)]
pub struct Joueur {
    name: String,
    point: i8,
    victoire: i8,
}

impl Joueur {
    ///Contructeur
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            point: 0,
            victoire: 0,
        }
    }
    pub fn get_name(&self) -> &str {
        return &self.name;
    }
    pub fn get_point(&self) -> i8 {
        return self.point;
    }
    pub fn get_victoire(&self) -> i8 {
        return self.victoire;
    }
    pub fn add_point(&mut self) {
        self.point += 1;
    }
    pub fn add_victoire(&mut self) {
        self.victoire += 1;
    }

    pub fn reset_point(&mut self) {
        self.point = 0;
    }

    pub fn reset_victoire(&mut self) {
        self.victoire = 0;
    }
    pub fn reset_all(&mut self) {
        self.reset_point();
        self.reset_victoire();
    }
}
