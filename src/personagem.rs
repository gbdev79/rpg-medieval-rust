use crate::jogar_dados::*;

#[derive(Debug)]
pub enum ClasseJogador {
    Guerreiro,
    Paladino,
    Druida,
    Mago,
    Cidadão,
}

pub struct Jogador {
    pub nome: String,
    pub max_hp: u32,
    pub hp: u32,
    pub max_mp: u32,
    pub mp: u32,
    // inventario: ,
    pub classe: ClasseJogador,
}

impl Jogador {

    // funciona como um construtor em Rust
    pub fn novo(nome_escolhido: String, input_vocacao: &str) -> Jogador {

        // Declara as variáveis que vão para o struct, o Match substitui o uso de if`s
        let (classe, max_hp, max_mp) = match input_vocacao.trim() {
            "1" => (ClasseJogador::Guerreiro, 200, 30),
            "2" => (ClasseJogador::Paladino, 150, 100),
            "3" => (ClasseJogador::Druida, 80, 120),
            "4" => (ClasseJogador::Mago, 60, 200),
            _ => (ClasseJogador::Cidadão, 100, 50),
        };

        Jogador {
            nome: nome_escolhido,
            max_hp,
            hp: max_hp,
            max_mp,
            mp: max_mp,
            classe
        } // a ultinha linha sem ; de um fn é o return

    }

    pub fn atacar(&mut self) {

        let dado = rolar_d6();

        if dado <= 2 {
            println!("> Você errou o ataque!")
        } else if dado == 3 {
            println!("> Você acertou de raspão")
        } else if dado == 4 || dado == 5 {
            println!("> Você acertou o ataque")
        } else {
            println!("> Você acertou um ataque crítico")
        }

    }

}