use std::io;
use crate::personagem::*;
use crate::enemies::*;

pub fn iniciar_combate(personagem: &mut Jogador, inimigo: &mut Inimigo) {
    println!("\n⚔️ --- O COMBATE COMEÇOU! --- ⚔️");
    inimigo.anunciar();

    while personagem.hp > 0 && inimigo.hp > 0 {  
        println!("\n {}: HP {}/{}", personagem.nome, personagem.hp, personagem.max_hp);
        println!("\n Escolha sua ação:");
        println!("1. Atacar");
        println!("2. Tentar fugir");

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Erro");

        match escolha.trim() {
            "1" => { 
                personagem.atacar(inimigo);

                if inimigo.hp == 0 {
                    println!("\n 🏆 Vitória! Você derrotou o {}!", inimigo.nome);
                    break;
                }

                println!("\n {} se prepara para contra-atacar...", inimigo.nome);
                inimigo.ataque(personagem);

                if personagem.hp == 0 {
                    println!("\n ☠️ Você foi derrotado em combate...");
                    break;
                }
            }

            "2" => {
                println!("Você conseguiu fugir covardemente!");
                break;
            }

            _ => println!("Ação inválida! Você perdeu seu turno hesitando..."),
        }


    }
}

