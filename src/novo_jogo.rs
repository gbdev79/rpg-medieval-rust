// biblioteca padrão de input/output
use std::io;

// biblioteca para formatação do texto que aparece no console
use textwrap::fill;

// importar funções de outros arquivos .rs para o projeto (na main, usamos o nome do projeto, em outro arquivos usamos "crate" para referenciar o arquivo lib.rs)
use crate::cidade::*;
use crate::personagem::*;

// Cria o personagem e envia os dados para a Struct Jogador em Personagem.rs
pub fn criar_personagem() -> Jogador {

    // variáveis para receber os inputs do personagem
    let mut nome_personagem = String::new();
    let mut input_vocacao = String::new();

    println!(">>> Qual é o nome do seu personagem?");
        
        io::stdin()
            .read_line(&mut nome_personagem)
            .expect("Falha");

        println!(">>> Qual vocação você deseja ser?\n
        1. Guerreiro
        2. Paladino
        3. Druida
        4. Mago
        ");
        
        io::stdin()
            .read_line(&mut input_vocacao)
            .expect("Falha");

        // Declara as variáveis que vão para o struct, o Match substitui o uso de if`s
        let (classe, hp, mp) = match input_vocacao.trim() {
            "1" => (String::from("Guerreiro"), 200, 30),
            "2" => (String::from("Paladino"), 150, 100),
            "3" => (String::from("Druida"), 80, 120),
            "4" => (String::from("Mago"), 60, 200),
            _ => (String::from("Cidadão"), 100, 50),
        };

        // remove o \n do input
        let nome_limpo = nome_personagem.trim().to_string();

        // retorna a tupla pro struct
        Jogador { nome: nome_limpo, hp, mp, classe }
}

// Inicia o jogo
pub fn nova_historia(personagem: &Jogador) {
    println!("> Olá, {}... O grande {}!", personagem.nome, personagem.classe);
    println!();

    // fill é o método da biblioteca de formatação de texto para o console
    println!("{}", fill("Seja bem-vindo a Eldoryan, um reino ancestral onde a própria essência da magia pulsa através da terra e ecoa nos ventos.", 70));
    println!();
    println!("{}", fill("Você acaba de adentrar um império de magnitude incomparável, moldado por eras de glória e lendas vivas, onde torres colossais arranham os céus e ruínas místicas aguardam para sussurrar seus segredos.", 70));
    println!();

    println!("{}", fill("> A sua esquerda está o caminho da cidade e a sua direita o bosque da floresta", 70));
    println!();
    println!("{}", fill(">>> Para onde você quer ir? \n
    1. Para a cidade \n
    2. Para o bosque", 70));

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha");

    if input.trim() == "1" {
        ir_para_cidade();
        input.clear();
    }

    if input.trim() == "2" {
        println!("Você foi em direção ao bosque...")
    }
}