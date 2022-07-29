const ROUNDS: i8 = 8;

pub fn encrypt(data: u64, key: u64) -> u64 {
    let (mut left, mut right) = split(data);

    for round in 0..ROUNDS {
        let k = (key >> (32/ROUNDS * round)) as u8; //u8 = u64/ROUNDS
        (left, right) = swap(left ^ prng(right, k), right)
    }

    combine(swap(left, right))
}

pub fn decrypt(cipher: u64, key: u64) -> u64 {
    let (mut left, mut right) = split(cipher);

    for round in (0..ROUNDS).rev() {
        let k = (key >> (32/ROUNDS * round)) as u8; //u8 = u64/ROUNDS
        (left, right) = swap(left ^ prng(right, k), right)
    }

    combine(swap(left, right))
}

fn split(x: u64) -> (u32, u32) {
     ((x >> 32) as u32, x as u32)
}

fn combine((l, r): (u32, u32)) -> u64 {
    ((l as u64) << 32) + (r as u64)
}

fn swap(a: u32, b:u32) -> (u32, u32) {
    (b, a)
}

/*
    Psuedo-random number generator
*/
fn prng(seed: u32, c: u8) -> u32 {
    let mut x: u64 = seed as u64;
    for _ in 0..4 {
        x = (7919*x + (c as u64)) % u32::MAX as u64;
    }
    x as u32
}

