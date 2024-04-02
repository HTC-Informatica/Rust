fn main() {
    // let mut x = 5;
    // println!("O valor de x é: {}", x);
    // x = 6;
    // println!("O valor de x é: {}", x);

    // TESTE DE SHADOWING 

        // let x = 5;
        // let x = x + 1;
        // let x = x * 2;
        // println!("O valor de x é: {}", x);
    
    // --------

    // TESTES DE TUPLAEROS DE DESESTRUTURAÇAO 
    // QUE SEPARA-OS EM TRES PARTE 

        // let tup = (500, 6.4, 1);
        // let (x, y, z) = tup;
        // println!("O valor do y é: {}", y);

    // --------
    
    // Nesta parte mostra como  acessar  cada valor da 
    // tupla usando (.) e o endereço do valor
    
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let quinhentos = x.0;
    let seis_ponto_quatro = x.1;
    let um = x.2;

    // --------

    let a = [1, 2, 3, 4, 5];
    let indice = 4;
    let elemento = a[indice];
    println!("O valor do elemento é: {} Teste de tuplaeros --> {}, {}. {}", 
    elemento, quinhentos, seis_ponto_quatro, um);

}

