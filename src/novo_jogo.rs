// biblioteca padrão de input/output
use std::io;

// biblioteca para formatação do texto que aparece no console
use textwrap::fill;

// importar funções de outros arquivos .rs para o projeto (na main, usamos o nome do projeto, em outro arquivos usamos "crate" para referenciar o arquivo lib.rs)
use crate::cidade::*;

// fn que cria uma tupla com 2 argumentos (nome e vocação). "pub" permite ser acessada por outro arquivo
pub fn criar_personagem() -> (String, String) {

    let mut nome_personagem = String::new();
    let mut vocacao = String::new();

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
            .read_line(&mut vocacao)
            .expect("Falha");

        let escolha = vocacao.trim();
        
        if escolha == "1" {

            // uma variavel que recebe input precisa de .clear() para receber outros valores
            vocacao.clear();

            // String::new() insere uma string dinâmica vazia na variável, mas "Guerreiro" é uma string fixa, que precisa ser convertida por push.str em array, para ser realocada na variável
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

        // remove o \n do input
        let nome_limpo = nome_personagem.trim().to_string();
        let vocacao_limpa = vocacao.trim().to_string();

        // retorna a tupla pra "fora" da fn
        (nome_limpo, vocacao_limpa)
}

// essa função recebe &str para sinalizar que variáveis de fora que não pertencem a ela serão usadas "emprestadas"
pub fn nova_historia(nome: &str, vocacao: &str) {
    println!("> Olá, {}... O grande {}!", nome, vocacao);
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
}