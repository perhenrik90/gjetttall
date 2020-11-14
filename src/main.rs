use std::io;
use rand::Rng;


fn main() {
    println!("Gjett et tall mellom 1 og 10!");

    println!("Hva gjetter du?");

    // Lag en lås
    let mut fortsett = true;
    
    // Lag hemmelig tall
    let hemmelighet = rand::thread_rng().gen_range(1, 10);


    // Teller
    let mut antall = 0;
    
    while fortsett {

	// Les inn tall
	let mut gjettet_str = String::new();
	io::stdin()
	    .read_line(&mut gjettet_str)
	    .expect("Feilet aa lese linja!");

	let gjettet: i32 = gjettet_str.trim().parse().expect("Kunne ikke tolke tall");

	if hemmelighet == gjettet {
	    println!("Bra. Du gjettet riktig! Tallet var {}", gjettet_str);
	    fortsett = false;
	}
	else{
	    println!("Sorry, du gjettet feil.\nPrøv igjen!");
	    antall += 1;
	}

    }

    println!("Hemmelig tall var: {}", hemmelighet);
    println!("Du brukte {} forsøk på å gjette!", antall);
}
