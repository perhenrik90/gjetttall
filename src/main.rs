use std::io;
use rand::Rng;


fn main() {
    println!("Gjett et tall!");

    println!("Hva er det du gjetter?");

    let mut gjettet = String::new();
    let hemmelighet = rand::thread_rng().gen_range(1, 10);
    
    io::stdin()
	.read_line(&mut gjettet)
	.expect("Feilet aa lese linja!");

    println!("Du gjettet: {}", gjettet);
}
