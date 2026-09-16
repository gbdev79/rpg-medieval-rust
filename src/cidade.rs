// importar funções de outros arquivos .rs para o projeto (na main, usamos o nome do projeto, em outro arquivos usamos "crate" para referenciar o arquivo lib.rs)
<<<<<<< Updated upstream
use crate::agir::*;
=======
use crate::personagem::*;
use crate::enemies::*;
use crate::combate::*;
>>>>>>> Stashed changes

pub fn ir_para_cidade() {
    println!("> Você foi em direção à cidade...");
    
    let mut inimigo = sortear_inimigo(Local::Cidade);

    iniciar_combate(personagem, &mut inimigo);

<<<<<<< Updated upstream
    io::stdin()
        .read_line(&mut input)
        .expect("Falha");

    if input.trim() == "1" {
        atacar();
=======
    if personagem.hp > 0 {
        println!("Após a batalha, você continua sua jornada e finalmente chega a cidade!")
>>>>>>> Stashed changes
    }

}