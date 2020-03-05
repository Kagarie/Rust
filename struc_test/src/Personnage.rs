#[derive(Debug)]
pub struct Personnage{
    nom: &'static str,
    // point de vie
    pts: i8,
    att: i8,
    def: i8
}

impl Personange{
    fn persoCrate(para : &Personnage) -> Personnage{
    para.nom = "personnage",
    para.pts = 20,
    para.att = 10,
    para.def = 10,
    }

}
