extern  Personnage;

fn main() {
    let perso = Personnage{
    nom:"test",
    pts:20,
    attaque:5,
    def:5,
    };
    println!("{:?}", perso);
}
