use rand::Rng;
use std::collections::HashMap;
use std::io;

fn main() {
    println!("Jeu de la pierre feuille ciseaux\n");

    //HashMap des choix de l'utilisateur
    let mut map: HashMap< i32, &str> = HashMap::new();
    map.insert(1, "Pierre");
    map.insert(2 ,"Feuille");
    map.insert(3, "Ciseaux");

    let (mut cpt_victoire_ordi , mut cpt_victoire_user )= (0,0) ;

    //Boucle du jeu
    loop{
        //Choix aléatoire de l'ordi
        let ordi : i32 = rand::thread_rng().gen_range(1..3);

        let mut input = String::new();

        //Saisie de l'utilisateur
        println!("Pierre [1],Feuille [2] ,Ciseaux[3]\nSaisie :");
        io::stdin().read_line(&mut input).expect("error reading input");

        /*On parse la saisie de l'utilisateur en nombre
        Si on arrive pas à le parser ou que le nombre n'est pas compris entre 1 et 3 on revient
        au début de la boucle
         */
        let number :i32  = match input.trim().parse()  {
            Ok(number)=>
            //Cas si la fonction  parse réussi mais que le nombre n'est pas compris entre 1 et 3
                if number > 3 || number < 1 {
                    println!("La saisie doit être soit 1 , 2 ou 3");
                    continue;
                 }
                //Si tout ce passe bien on retourne notre nombre entre 1 et 3
                else {
                    number
                },
            //Cas d'erreur si la fonction parse échoue on retourne au début de la boucle
            Err(_) => {
                println!("Saisie: error");
                continue;
            }
        };

        //Cas ou l'utilisateur et l'ordi on le même choix
        if ordi==number {println!("Manche nul"); }
            //Cas ou l'odi gagne
        else if ordi == 1 && number ==3  || ordi == 2 && number ==1  || ordi == 3 && number ==2 {
            println!("Manche Perdu !");
            //Afficahge des choix de l'ordi et de l'utilisateur
            println!("ordi : {} , vous : {} \n",map[&ordi],map[&number]);
            cpt_victoire_ordi +=1;
        }
            //Cas ou l'utilisateur gagne
        else if number == 1 && ordi ==3  || number == 2 && ordi ==1  || number == 3 && ordi ==2 {
            println!("Manche Gagné !");
            println!("ordi : {} , vous : {} \n",map[&ordi],map[&number]);
            cpt_victoire_user+=1;
        }

        if cpt_victoire_user <3 && cpt_victoire_ordi < 3 {
            println!("Vous avez {} points",cpt_victoire_user);
            println!("L'ordi a {} points",cpt_victoire_ordi);
            println!("\n##########################################################################\n")
        }

        //Cas final si l'utilisateur ou l'ordi et à 3 victoires fin du jeu
        if cpt_victoire_user == 3 {
            println!("Vous avez gagné !",);
            break;
        }else if cpt_victoire_ordi == 3 {   println!("Vous avez perdu !" );
            break;}
        }
    }
