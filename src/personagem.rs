use crate::jogar_dados::*;
use crate::enemies::*;


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

    pub fn atacar(&self, alvo: &mut Inimigo) {

        let dado = rolar_d20();

        let (multiplicador, mensagem) = match dado {
            1..=2 => (0, "Você errou o golpe miseravelmente!"),
            3..=4 => (1, "Você acertou de raspão, causando dano mínimo."),
            5..=15 => (3, "Você acertou um golpe firme!"),
            16..=17 => (5, "Um ataque poderoso acertou em cheio!"),
            18..=19 => (10, "ATAQUE CRÍTICO! Golpe devastador!"),
            20 => (alvo.max_hp, "HIT KILL! Um golpe lendário decapitou o inimigo!"),
            _ => (0, "Algo estranho aconteceu..."),
        };
        let dano = multiplicador * dado;

        println!("> {}", mensagem);

        if dano > 0 {
            println!("> Causou {} de dano a {}!", dano, alvo.nome);
            alvo.receber_dano(dano);
        }

    }
    pub fn receber_dano(&mut self, quantidade_hp: u32 )  {
        self.hp = self.hp.saturating_sub(quantidade_hp);


        if self.hp == 0 {
            println!("{} foi derrotado!", self.nome);
        }else{
            println!("{}, HP {}/{}", self.nome, self.hp, self.max_hp);
        }
    }
}