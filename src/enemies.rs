use crate::jogador::Jogador;

// Pesquisei o equivalente de classes em rust, o struct define as propriedades, 
// e o impl define os métodos



pub enum ClasseInimigo{
    Humanoide(Humanoide),
    Aladas(AladasEDragoes),
    Fera(FeraTerrestre),
}

enum Humanoide{
    Goblin, 
    Ogro,
    Minotauro, 
    Ciclope,
    Troll
}

enum AladasEDragoes{           
    Dragao,
    Wyvern,
    Grifo,
    Gargula,
    Harpia
}

enum FeraTerrestre{
    Quimera,
    Manticora,
    Basilisco,
    Cerberus,
    Warg
}

pub struct Inimigo{
    nome: String, 
    hp: u32,
    ataque: u32,
    tipo_de_ataque: String,
    tipo: ClasseInimigo
}


impl Inimigo{

    fn ataque(&mut self, alvo: &mut Jogador ){
        alvo.hp = alvo.hp.saturating_sub(self.ataque);
        //self.ataque é literalmente um valor da instancia Inimigo (inimigo.ataque(u32))

        if(alvo.hp == 0){
            println!("{} sucumbiu diante do ataque de {}...", alvo.nome, self.nome);
        }else{
            println!("{} atacou e causou {} de dano!", self.nome, self.ataque);
        }
    }



    fn receber_dano(&mut self, quantidade_hp: u32 )  {
        self.hp = self.hp.saturating_sub(quantidade_hp);  //proteção pra n virar negativo


        if(self.hp == 0){
            println!("{} foi derrotado!", self.nome);
        }else{
            println!("{} agora tem {} de vida", self.nome, self.hp);
        }
    }
}





// trait (comportamento compartilhado)



// let mut goblin = Inimigo { nome: "Goblin".to_string(), hp: 30, ataque: 5, ... };
// let mut jogador = Jogador { nome: "Eduardo".to_string(), hp: 100, ... };

// goblin.ataque(&mut jogador);