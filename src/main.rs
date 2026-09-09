// biblioteca padrão de input/output
use std::io;

// exporta outros arquivos .rs para o projeto
mod novo_jogo;

fn main() {
    println!("=== RPG MEDIEVAL === \n");

    println!("Menu Principal: \n 
    1. Novo jogo
    2. Carregar jogo
    ");

    // cria uma variavel mutável (mut) para receber o input do usuário
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha");

    // ".trim()" remove o \n que vem junto com o input do usuário    
    if input.trim() == "1" {

        // recebe a tupla criada em criar_personagem() com nome e vocação além de chamar a fn
        let (nome, vocacao) = novo_jogo::criar_personagem();

        // chama a fn e permite que ela use a tupla criada anteriormente
        novo_jogo::nova_historia(&nome, &vocacao);

        // return
        (nome, vocacao)

    } else if input.trim() == "2" {
        println!("Não existe jogo salvo.");
        return;

      } else {
        println!("Opção inválida.");
        return;
    };
    
}

