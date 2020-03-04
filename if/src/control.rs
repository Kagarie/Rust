#![feature(plugin)]
extern crate scan;
//use scan::Scan;


fn main() {
    let mut scanner = scan::from_stdin();
    let my_int = scanner.next::<i32>();

    if my_int < 10 {
        println!("false x = {}",my_int);
    }else{
        println!("true number = {}", my_int);
    }
}
