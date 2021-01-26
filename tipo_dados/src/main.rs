fn main() {
    //O compilador pode aferir qual o tipo de dado de acordo com o valor atribuido a variável
    //Existem duas categorias de dados: Escalares e Compostos

    println!("Dados escalares: representam valores únicos.");
    //Inteiros
    //Um inteiro é um número sem ponto flutuante
    //Pode ser unsigned(sem sinal +-) ou signed(com sinal +-); unsigned começa com u signed com i
    //pode ser: i ou u que por sua vez pode ser 8 16 32 64 bit;
    //O default do Rust é i32
    let signed_int: i32 = -10;
    let unsigned_int: u32 = 10;
    println!("int signed={}; unsigned={}", signed_int, unsigned_int);

    //Floats
    //podem ser f32 ou f64; o padrão do Rust é f64
    let float_64 = 3.14;
    let float_32: f32 = 1.5;
    println!("float f64={}; f32={}", float_64, float_32);

    //Boolean
    //especificado por bool pode ser true ou false
    let bool_1 = true;
    let bool_2: bool = false;
    println!("bool b1={}; b2={}", bool_1, bool_2);

    //Char
    //especificado por char; representa um caracter; usa-se aspas simples 
    let char_1 = 'R';
    let char_2 = 'ç';
    println!("char char_1={}; char_2={}", char_1, char_2);

    println!("Dados compostos: agrupam múltiplos valores em um tipo.");
    println!("Tuplas: let tupla:(i32, f64) = (10, 1.5)");
    let tupla: (i32, f64, &str) = (10, 1.4, "Rust");
    println!("Qtd: {}; Vl: {}; Lang: {}\n", tupla.0, tupla.1, tupla.2);
    
    println!("\nArrays: let a = [1, 2, 3, 4] ou let a: [i32; 5]= [1, 2, 3, 4, 5]");
    let a = [1, 2, 3, 4]; 
    let a:[i32; 5] = [1, 2, 3, 4, 5];
    let meses = ["Jan", "Fev", "Mar", "Abr", "Mai", "Jun", "Jul", "Ago", "Set", "Out", "Nov", "Dez"];
    println!("Podemos inicializar o array com um valor padrão: let a = [3; 5]");
    let xpto = [3; 5];
    println!("{}", xpto[1]);    
}

