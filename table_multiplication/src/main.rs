use std::io::stdin;

fn main() {
    loop {
        let mut input = String::new();

        println!("Entrée un nombre");
        stdin().read_line(&mut input).expect("Error reading");

        let number: i32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("\nVeuillez entrer un nombre");
                continue;
            }
        };

        for i in 1..=10 {
            println!("{} x {} = {}", number, i, (&number * i));
        }
        println!();
    }
}
