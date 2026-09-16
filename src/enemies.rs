use crate::personagem::*;
use rand::Rng;

// // Pesquisei o equivalente de classes em rust, o struct define as propriedades, 
// // e o impl define os métodos

#[derive(Clone, Copy, Debug)]
pub enum Local {
    Bosque,
    Cidade,
    Caverna,
}

#[derive(Clone, Copy, Debug)]
pub enum ClasseInimigo{
    Humanoide(Humanoide),
    Aladas(AladasEDragoes),
    Fera(FeraTerrestre),
}

#[derive(Clone, Copy, Debug)]
pub enum Humanoide{
    Goblin, 
    Ogro,
    Minotauro, 
    Ciclope,
    Troll
}

#[derive(Clone, Copy, Debug)]
pub enum AladasEDragoes{           
    Dragao,
    Wyvern,
    Grifo,
    Gargula,
    Harpia
}

#[derive(Clone, Copy, Debug)]
pub enum FeraTerrestre{
    Quimera,
    Manticora,
    Basilisco,
    Cerberus,
    Warg
}

pub struct Inimigo{
    pub nome: String, 
    pub max_hp: u32,
    pub hp: u32,
    pub ataque: u32,
    pub tipo_de_ataque: String,
    pub tipo: ClasseInimigo,
    pub texto: String
}


impl Inimigo{

    pub fn novo(tipo: ClasseInimigo) -> Inimigo {
        let (nome, max_hp, ataque, tipo_de_ataque, texto) = match tipo {
            ClasseInimigo::Humanoide(Humanoide::Goblin) => {
                (String::from("Goblin"), 60, 5, String::from("Físico"), String::from("Um Goblin selvagem apareceu, com uma faca entre os dentes e sangue nos olhos!"))
            }
            ClasseInimigo::Humanoide(Humanoide::Troll) => {
                (String::from("Troll"), 80, 9, String::from("Físico"), String::from("O chão estremeu e os pássaros voaram, quando um Troll selvagem surgiu!"))
            }
            _ => {
                (String::from("Criatura desconhecida"), 10, 2, String::from("Físico"), String::from("Uma criatura desconhecida surgiu!"))
            }
        };

        Inimigo {
            nome,
            max_hp,
            hp : max_hp,
            ataque,
            tipo_de_ataque,
            tipo,
            texto,
        }
    }

    pub fn anunciar(&self) {
        println!("\n⚠️ {}", self.texto);
        println!("Status: {} | HP: {}/{}", self.nome, self.hp, self.max_hp);
    }

    pub fn ataque(&mut self, alvo: &mut Jogador){
        alvo.hp = alvo.hp.saturating_sub(self.ataque);
        //self.ataque é literalmente um valor da instancia Inimigo (inimigo.ataque(u32))

        if alvo.hp == 0 {
            println!("{} sucumbiu diante do ataque de {}...", alvo.nome, self.nome);
        }else{
            println!("{} atacou e causou {} de dano!", self.nome, self.ataque);
        }
    }

    pub fn receber_dano(&mut self, quantidade_hp: u32 )  {
        self.hp = self.hp.saturating_sub(quantidade_hp);  //proteção pra n virar negativo


        if self.hp == 0 {
            println!("{} foi derrotado!", self.nome);
        }else{
            println!("{}: HP {}/{}", self.nome, self.hp, self.max_hp);
        }
    }
}

pub fn sortear_inimigo(local: Local) -> Inimigo {

    let tabela_de_encontros = match local {
        Local::Cidade => vec![
            ClasseInimigo::Humanoide(Humanoide::Goblin),
            ClasseInimigo::Humanoide(Humanoide::Troll),
        ],
        _ => vec! {
            ClasseInimigo::Humanoide(Humanoide::Goblin),
        },
    };

    let mut rng = rand::thread_rng();
    let indice = rng.gen_range(0..tabela_de_encontros.len());
    let tipo_sorteado = tabela_de_encontros[indice];

    Inimigo::novo(tipo_sorteado)

}



// // trait (comportamento compartilhado)



// // let mut goblin = Inimigo { nome: "Goblin".to_string(), hp: 30, ataque: 5, ... };
// // let mut jogador = Jogador { nome: "Eduardo".to_string(), hp: 100, ... };

// // goblin.ataque(&mut jogador);