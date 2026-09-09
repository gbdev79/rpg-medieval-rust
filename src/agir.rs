use crate::jogar_dados::*;

pub fn atacar() {

    let dado = rolar_d6();
    if dado <= 2 {
        println!("Você errou o ataque!")
    } else if dado == 3 {
        println!("Você acertou de raspão")
    } else if dado == 4 || dado == 5 {
        println!("Você acertou o ataque")
    } else {
        println!("Você acertou um ataque crítico")
    }

}