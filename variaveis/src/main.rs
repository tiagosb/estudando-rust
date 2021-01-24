fn main() {
    //Constantes não podem ser tornadas mutaveis; devem ser tipadas; por convenção usam-se
    //maiúsculas para nomear
    const MAXIMO: i32 = 10;

    //A palavra chave mut torna a variável mutável
    let mut numero = 10;
    println!("numero = {}", numero);
    numero = 20;
    println!("numero agora é = {}", numero);

    //Shadowing - Sobreescrevendo uma variável podemos alterar valor e até mesmo o tipo
    //o mut não permitiria isso
    let espacos = "   ";
    let espacos = espacos.len();
    println!("Espaçamento: {}; O máximo é: {}", espacos, MAXIMO);
}
