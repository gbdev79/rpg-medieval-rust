use std::io;

pub fn criar_personagem() -> (String, String) {

    let mut nome_personagem = String::new();
    let mut vocacao = String::new();

    println!("Qual é o nome do seu personagem?");
        io::stdin()
            .read_line(&mut nome_personagem)
            .expect("Falha");

        println!("Qual vocação você deseja ser?\n
        1. Guerreiro
        2. Paladino
        3. Druida
        4. Mago
        ");
        io::stdin()
            .read_line(&mut vocacao)
            .expect("Falha");

        let nome_limpo = nome_personagem.trim().to_string();
        let vocacao_limpa = vocacao.trim().to_string();

        (nome_limpo, vocacao_limpa)
}