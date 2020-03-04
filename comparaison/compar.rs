use std::io;

fn comparaison(first: String , seconde: String){
if first==seconde
{
    print!("les chaines sont identiques\n");
}
else{ 
    print!("les chaines ne sont pas identique\n")
}
}

fn main() {
    let mut first = String::new();
    let mut seconde = String::new();
        print!("Première chaine:\n");
        io::stdin().read_line(&mut first).expect("Failed to read line");
        print!("Seconde chaine:\n");
        io::stdin().read_line(&mut seconde).expect("Failed to read line");
        comparaison(first,seconde);
}
