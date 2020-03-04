use std::io;

fn comparaison(first: String , second: String){
if first==second{
    print!("les chaines sont identiques\n");
}
else{ 
    print!("les chaines ne sont pas identique\n")
}
}

fn main() {
    print!("Première chaine :\n");
    let mut first = String::new();
    io::stdin().read_line(&mut first).expect("Failed to read line");
    print!("Seconde chaine :\n");
    let mut second=String::new();
    io::stdin().read_line(&mut second).expect("Failed to read line"); 
    comparaison(first,second);
}
