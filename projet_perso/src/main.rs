use rand::random;

struct User {
    nom: &'static str,
    age: i32,
}

impl User {
    /**
    *Constructeur
    **/
    pub fn new(nom: &'static str, age: i32) -> Self {
        User { nom, age }
    }

    /**
    *Getteur
    **/
    pub fn get_nom(&self) -> &'static str {
        self.nom
    }
    pub fn get_age(&self) -> i32 {
        self.age
    }

    /**
    *Setteur
    **/
    pub fn set_nom(&mut self, nom: &'static str) {
        self.nom = nom;
    }
    pub fn set_age(&mut self, age: i32) {
        self.age = age;
    }


    /**
    *To string
    **/
    pub fn to_string(&self) -> () { println!("bonjour je suis {} et j'ai {} ans", self.nom, self.age); }
}

fn main() {
    //generation d'un nombre aleatoire
    let mut x = random();

    //je caste le random en 8 bits puis en 32 pour avoir un age plausible et le 32 bits pour respecter ma structure ou age prends un entier de 32 bits
    x = x as i8 as i32;
    //si mon x est negatif je le rends positif
    if x < 0 { x *= -1; }

    let mut personne = User::new("Albert", x);
    //affichage des getteurs
    println!("nom : {}", User::get_nom(&personne));
    println!("age : {}", User::get_age(&personne));

    User::to_string(&personne);

    //utilisation des setteurs
    User::set_nom(&mut personne, "Macron");
    User::set_age(&mut personne, 20);

    User::to_string(&personne);
}

#[test]
fn test() {
    //creation d un user
    let mut p = User::new("richard", 32);
    let mut name = "richard";
    let mut x = 32;

    //utilisation de assert
    assert_eq!(User::get_nom(&p), name, "get nom {} avec {}", User::get_nom(&p), name);
    assert_eq!(User::get_age(&p), x, "get age {} avec {}", User::get_age(&p), x);

    User::set_nom(&mut p, "albert");
    User::set_age(&mut p, 20);
    name = "albert";
    x = 20;
    assert_eq!(User::get_nom(&p), name, "get nom {} avec {}", User::get_nom(&p), name);
    assert_eq!(User::get_age(&p), x, "get age {} avec {}", User::get_age(&p), x);
}
