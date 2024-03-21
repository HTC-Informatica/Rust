extern crate rand;

use std::io;
use std::cmp::Odering;
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

    match palpite.cmp(&numero_secreto) {

        Odering::Less => println!("Muito baixo!");
        Odering::Greater => println!("Muito alto!");
        Odering::Equal => println!("você acertou!");
    } 
}
// Pausa::Comparando o número secreto
