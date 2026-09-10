use rand::Rng; // números aleatórios

pub fn rolar_d6() -> i32 { // declara o tipo Int para o return da função
    let mut rng = rand::thread_rng(); // cria o gerador de números
    let chance = rng.gen_range(1..=6); // defini o range dos números

    println!(">> Rolando D6...");

    println!("> Você tirou um {} no dado", chance);

    chance // indica o valor retornado
}