#[derive(Clone)]
pub struct Personne {
    name: String,
    age: i8,
}

impl Personne {

    ///Constructeur
    pub fn new(name: String, age: i8) -> Self {
        Self {
            name: name,
            age: age,
        }
    }

    ///Getteur et setteur
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
    pub fn get_age(&self) -> i8 {
        return self.age.clone();
    }
    pub fn set_name(&mut self, s: String) {
        self.name = s;
    }
    pub fn set_age(&mut self, n: i8) {
        self.age = n;
    }
    pub fn to_string(&self) {
        println!("Personne [name : {} , age : {} ]", self.name, self.age);
    }

    ///Ajoute +1 à l'age de la personne
    pub fn viellir(&mut self) {
        self.age += 1;
    }

    ///Return vrai si deux personne et le même nom et le même age
    pub fn eq(&self, p: &Personne) -> bool {
        if String::eq(&self.name, &p.name) && self.age == p.age {
            return true;
        }
        return false;
    }

    ///Return vrai si la personne à 18 ans ou plus
    pub fn peut_conduire(&self) -> bool {
        if self.age < 18 {
            return false;
        } else {
            return true;
        }
    }
}
