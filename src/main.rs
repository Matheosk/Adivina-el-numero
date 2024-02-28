use rand::Rng;
use std::io;

fn main() {
    // Crea variables
    let mut rng = rand::thread_rng();
    let numero_aleatorio: u8 = rng.gen_range(1..10);
    let mut numero: String = String::new();
    let stdin = io::stdin();

    // Empieza el juego
    println!("Adivina el numero aleatorio del 1 al 10");

    stdin.read_line(&mut numero).unwrap();
    let numeroN: u8 = numero.trim().parse().unwrap();

    if numero_aleatorio == numeroN {
        println!("Acertaste");
    } else {
        println!("El numero era {numero_aleatorio}");
    }
}
