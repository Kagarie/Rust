#[derive(Clone)]
pub struct Personne {
    name: String,
    age: i8,
}

impl Personne {
    pub fn new(name: String, age: i8) -> Self {
        Self {
            name: name,
            age: age,
        }
    }

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

    pub fn viellir(&mut self) {
        self.age += 1;
    }

    pub fn eq(&self, p: &Personne) -> bool {
        if String::eq(&self.name, &p.name) && self.age == p.age {
            return true;
        }
        return false;
    }

    pub fn peut_conduire(&self) -> bool {
        if self.age < 18 {
            return false;
        } else {
            return true;
        }
    }
}
