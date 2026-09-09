use std::io;
use textwrap::fill;

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

        let escolha = vocacao.trim();
        
        if escolha == "1" {
            vocacao.clear();
            vocacao.push_str("Guerreiro");
        } else if escolha == "2" {
            vocacao.clear();
            vocacao.push_str("Paladino");
        } else if escolha == "3" {
            vocacao.clear();
            vocacao.push_str("Druida");
        } else if escolha == "4" {
            vocacao.clear();
            vocacao.push_str("Guerreiro");
        }

        let nome_limpo = nome_personagem.trim().to_string();
        let vocacao_limpa = vocacao.trim().to_string();

        (nome_limpo, vocacao_limpa)
}

pub fn nova_historia(nome: &str, vocacao: &str) {
    println!("Olá, {}... O grande {}!", nome, vocacao);
    println!();

    println!("{}", fill("Seja bem-vindo a Eldoryan, um reino ancestral onde a própria essência da magia pulsa através da terra e ecoa nos ventos.", 70));
    println!();
    println!("{}", fill("Você acaba de adentrar um império de magnitude incomparável, moldado por eras de glória e lendas vivas, onde torres colossais arranham os céus e ruínas místicas aguardam para sussurrar seus segredos.", 70));
    println!();

    println!("{}", fill("A sua esquerda está o caminho da cidade e a sua direita o bosque da floresta", 70));
    println!();
    println!("{}", fill("Para onde você quer ir? \n
    1. Para a cidade \n
    2. Para o bosque", 70));

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha");

    if input.trim() == "1" {
        println!("Você foi em direção à cidade...");
    }
}