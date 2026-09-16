// importar funções de outros arquivos .rs para o projeto (na main, usamos o nome do projeto, em outro arquivos usamos "crate" para referenciar o arquivo lib.rs)
use crate::agir::*;

pub fn ir_para_cidade(personagem: &mut Jogador) {
    println!("> Você foi em direção à cidade...");
    
    let mut inimigo = sortear_inimigo(Local::Cidade);

    iniciar_combate(personagem, &mut inimigo);

    io::stdin()
        .read_line(&mut input)
        .expect("Falha");

    if input.trim() == "1" {
        atacar();
    }

}