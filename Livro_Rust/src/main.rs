extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Advinhe o Numero");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);
    println!("O númereo secreto é: {}", numero_secreto);

    println!("Digite o palpite");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite)
        .expect("Falha ao ler entrada");

    println!("você disse {}", palpite);
}
// Pausa::Comparando o número secret