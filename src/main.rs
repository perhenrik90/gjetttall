use std::io;
use rand::Rng;


fn main() {
    println!("Gjett et tall mellom 1 og 10!");

    println!("Hva gjetter du?");

    // Lag hemmelig tall
    let hemmelighet = rand::thread_rng().gen_range(1, 10);

    // Les inn tall
    let mut gjettet_str = String::new();


    io::stdin()
	.read_line(&mut gjettet_str)
	.expect("Feilet aa lese linja!");

    let gjettet: i32 = gjettet_str.trim().parse().expect("Kunne ikke tolke tall");

    if hemmelighet == gjettet {
	    println!("Bra. Du gjettet riktig! Tallet var {}", gjettet_str);
    }
    else{
	println!("Sorry, du gjettet feil.")
    }
    
    println!("Du gjettet: {}", gjettet);
    println!("Hemmelig tall var: {}", hemmelighet);
}
