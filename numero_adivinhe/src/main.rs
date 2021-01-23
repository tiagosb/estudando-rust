use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    
    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    println!("Adivinhe o número!");
    println!("Por favor, escolha um número:");

    loop {

        let mut palpite = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler a linha");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você escolheu: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Seu número é menor que o secreto!"),
            Ordering::Greater => println!("Seu número é maior que o secreto!"),
            Ordering::Equal => {
                println!("Parabéns, acertou!");
                break;
            }
        }
    }
}
