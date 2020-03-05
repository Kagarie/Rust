#[derive(Debug)]
struct Personnage {
	//les string ont besoin d avoir une duree de vie la connaissant pas pour ce programme j'ulisie " &'static " pour lui dire qu'iu moura à la fin du programme 
	nom: &'static str,
	//f32 => nombre flottant attendu il sera code sur 32 bits (f64 max, i => entier i128 max, u => unsigned u128 max)
	longueur: f32,
	diametre: f32
}

//implementation des fonctions qui seront SEULEMENT a personnage (un peux comme une classe en java)
impl Personnage {
	//volume pointe sur Personnage et recupere la structure en memoire 
	fn volume (vol : &Personnage ) -> f32{
	//je n'ai plus qu'a appeler les parametres de ma structure en faisant simplement vol. et faire le calcule
	(3.14*((vol.diametre/2.0)*(vol.diametre/2.0))*vol.longueur)/(1000000.0)
	}
}

fn main() {
	//instancions de la structure " Personnage "
	let perso1=Personnage {
		//Pour le sring en structure c est un peux plus delicat que pour le reste mais c est l une des facons la plus simple 
		//je peux faire simplement un " nom : " le nom " " car LA DEFINITION DE MA STRUCTURE ME LE PERMET
		nom: "popof",
		longueur: 142.0,
		diametre: 5.0,
	};
	
	//affichage test de ma structure grace à " {:?} " 
	println!("{:?}",perso1);

	//appelle de la fonction volume dans Personnage
	println!("Le volume de popof est de {} m3", Personnage::volume(&perso1));
}
