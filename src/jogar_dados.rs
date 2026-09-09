use std::io; // padrão I/O
use rand::Rng; // números aleatórios

pub fn rolar_d6() -> i32 {
    let mut rng = rand::thread_rng();
    let chance = rng.gen_range(1..=6);

    println!(">> Rolando D6...");

    println!("Você tirou um {} no dado", chance);

    chance
}