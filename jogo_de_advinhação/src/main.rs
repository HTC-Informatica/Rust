extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Advinhe o Numero");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);
    
    loop {
    
        println!("Digite o palpite");

        let mut palpite = String::new();

        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler entrada");

            let palpite: u32 = match palpite.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };


        println!("você disse {}", palpite);

        match palpite.cmp(&numero_secreto) {

            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
            println!("você acertou!");
            break;
            }
        }
    }
}
// Pausa::Jogo concluido! partindo para o cap 3
