const ROUNDS: i8 = 4;

pub fn encrypt(data: u64, key: u32) -> u64 {
    let (mut left, mut right) = split(data);

    for round in 0..ROUNDS {
        let k = (key >> (32/ROUNDS * round)) as u8; //u8 = u32/ROUNDS
        (left, right) = swap(left ^ round_function(right, k), right)
    }

    combine(swap(left, right))
}

pub fn decrypt(cipher: u64, key: u32) -> u64 {
    let (mut left, mut right) = split(cipher);

    for round in (0..ROUNDS).rev() {
        let k = (key >> (32/ROUNDS * round)) as u8; //u8 = u32/ROUNDS
        (left, right) = swap(left ^ round_function(right, k), right)
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
Xn+1 = (aXn + c) mod m
where X is the sequence of pseudo-random values
m, 0 < m  - modulus 
a, 0 < a < m  - multiplier
c, 0 ≤ c < m  - increment
x0, 0 ≤ x0 < m  - the seed or start value
*/
fn round_function(seed: u32, k: u8) -> u32 {
    let mut x: u64 = seed as u64;
    for _ in 0..16 {
        x = (7919*x + (k as u64)) % u32::MAX as u64;
    }
    x as u32
}

