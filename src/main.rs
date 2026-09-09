use std::io;
use textwrap::fill;

mod novo_jogo;

fn main() {
    println!("=== RPG MEDIEVAL === \n");

    println!("Menu Principal: \n 
    1. Novo jogo
    2. Carregar jogo
    ");



    let mut input = String::new();
    let mut nome = String::new();
    let mut vocacao = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha");

    if input.trim() == "1" {
        
        let (nome_criado, vocacao_criada) = novo_jogo::criar_personagem();
        nome = nome_criado;
        vocacao = vocacao_criada;
    } else {
        println!("Não existe jogo salvo.");
        return;
    }

    println!("Olá, {}!", nome);
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

    input.clear();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha");

    if input.trim() == "1" {
        println!("Você foi em direção à cidade...");
    }
    
}

