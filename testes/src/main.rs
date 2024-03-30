fn main() {
    let mut x = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    println!("O valor de x é: {}", x);

// TESTE DE SHADOWING 
        let x = 5;
    
        let x = x + 1;
    
        let x = x * 2;
    
        println!("O valor de x é: {}", x);
// --------

// TESTES DE TUPLAEROS DE DESESTRUTURAÇAO 
// QUE SEPARA-OS EM TRES PARTE 

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("O valor do y é: {}", y);
}

