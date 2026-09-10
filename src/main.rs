use std::io;

mod novo_jogo;

fn main() {
    println!(" ⚔️ === RPG MEDIEVAL ===⚔️ \n");

    println!("📜 Menu Principal: \n 
    1. 🆕 Novo jogo
    2. 💾 Carregar jogo
    ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha");

    if input.trim() == "1" {
        let (nome, vocacao) = novo_jogo::criar_personagem();
        novo_jogo::nova_historia(&nome, &vocacao);
        (nome, vocacao)

    } else if input.trim() == "2" {
        println!("Não existe jogo salvo.");
        return;

      } else {
        println!("Opção inválida.");
        return;
    };
    
}

