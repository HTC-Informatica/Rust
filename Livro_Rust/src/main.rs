use std::io;

fn main() {
    println!("Advinhe o Numero");
    println!("Digite o palpite");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite)
        .expect("Falha ao ler entrada");

    println!("você disse {}", palpie);
}
