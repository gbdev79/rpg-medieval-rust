// biblioteca padrão de input/output
use std::io;

// importar funções de outros arquivos .rs para o projeto (na main, usamos o nome do projeto, em outro arquivos usamos "crate" para referenciar o arquivo lib.rs)
use rustpg::novo_jogo::*; // * representar a importação de todas as funções do arquivo apontado

fn main() {

    println!(" ⚔️ === RPG MEDIEVAL ===⚔️ \n");

    println!("📜 Menu Principal: \n 
    1. 🆕 Novo jogo
    2. 💾 Carregar jogo
    ");

    // cria uma variavel mutável (mut) para receber o input do usuário
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha");

    // ".trim()" remove o \n que vem junto com o input do usuário    
    if input.trim() == "1" {

        let mut jogador = criar_personagem();

        // chama a fn e permite que ela use a tupla criada anteriormente
        nova_historia(&mut jogador);

    } else if input.trim() == "2" {
        println!("Não existe jogo salvo.");
        return;

      } else {
        println!("Opção inválida.");
        return;
    };
    
}

