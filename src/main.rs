use std::io;

fn main() {
    println!("@@@@@@@ CONVERSOR DE TEMPERATURA @@@@@@@");
    println!("Converter de Celsius para Fahrenheit e vice versa");

    println!("Escolha uma escala: f para converter de celsius para fahrenheit e c para converter de fahrenheit para celsius");
    let mut scale = String::new();

    io::stdin().read_line(&mut scale)
        .expect("Você deve selecionar uma opção válida.");

    let scale: String = scale.trim().parse().expect("Errado");

    if scale != "c" && scale != "f" {
        return println!("Você deve selecionar uma opção válida.");
    }

    println!("Insira a temperatura");
    let mut temp = String::new();

    io::stdin().read_line(&mut temp)
        .expect("Falha ao ler entrada.");

    let temp: f32 = temp
        .trim()
        .parse()
        .expect("Digite um número.");

    let res = converter(temp, scale);

    println!("Resultado da conversão: {} ", res);
}

fn converter(temp: f32, scale: String) -> f32 {
    if scale == "f" {
        temp * 1.8 + 32.0
    } else {
        (temp - 32.0) / 1.8
    }
}