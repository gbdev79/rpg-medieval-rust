// biblioteca padrão de input/output
use std::io;

// biblioteca para formatação do texto que aparece no console
use textwrap::fill;

// importar funções de outros arquivos .rs para o projeto (na main, usamos o nome do projeto, em outro arquivos usamos "crate" para referenciar o arquivo lib.rs)
use crate::agir::*;

pub fn ir_para_cidade() {
    println!("> Você foi em direção à cidade...");
    println!();
    println!("{}", fill("> O dia estava ensolarado e os passáros cantavam ao longo da estrada, quando de repente...", 70));
    println!();
    println!("{}", fill("> Um ladrão apareceu e apontando uma faca exclamou:\n 'Me entregue sua mochila ou morra!'", 70));
    println!();
    println!(">>> O que você quer fazer?\n
    1. Atacar
    2. Correr
    ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha");

    if input.trim() == "1" {
        atacar();
    }

}