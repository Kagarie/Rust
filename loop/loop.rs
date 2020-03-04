use std::{thread, time};

fn main(){
    let seconde = time::Duration::from_millis(1000);
    let now = time::Instant::now();
    loop{
        println!("again");
        
        thread::sleep(seconde);
    }
}
