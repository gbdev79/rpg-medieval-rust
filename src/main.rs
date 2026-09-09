use std::io;

mod novo_jogo;

fn main() {
    println!("=== RPG MEDIEVAL === \n");

    println!("Menu Principal: \n 
    1. Novo jogo
    2. Carregar jogo
    ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha");

    let (nome, vocacao) = if input.trim() == "1" {
        novo_jogo::criar_personagem()
    } else {
        println!("Não existe jogo salvo.");
        return;
    };

    novo_jogo::nova_historia(&nome, &vocacao);
    
}

