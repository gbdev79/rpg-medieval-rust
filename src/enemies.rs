// Monstros

// Pesquisei o equivalente de classes em rust, o struct define as propriedades, 
// e o impl define os métodos


// Humanoides e Monstros

struct ClasseInimigo{
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

// Aladas e Dragoes

enum AladasEDragoes{
    Dragao,
    Wyvern,
    Grifo,
    Gargula,
    Harpia
}

// Feras e Criaturas Terrestres

enum FeraTerrestre{
    Quimera,
    Manticora,
    Basilisco,
    Cerberus,
    Warg
}

struct Inimigo{
    nome: String, 
    hp: u32,
    ataque: String //aereo, terrestre, ambos
    tipo: ClasseInimigo
}







// trait (comportamento compartilhado)