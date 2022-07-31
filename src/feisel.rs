const ROUNDS: u8 = 8;
const KEY_SIZE: u8 = 64;

pub fn encrypt(data: u64, key: u64) -> u64 {

    cipher(data, key)
}

pub fn decrypt(encrypted_data: u64, key: u64) -> u64 {

    cipher(encrypted_data, reverse_key(key))
}

pub fn cipher(data: u64, key: u64) -> u64 {
    let (mut left, mut right) = split(data);

    for round in 0..ROUNDS {
        (left, right) = (right, left ^ prng(right, subkey(key, round)));
    }

    combine((right, left))
}

pub fn reverse_key(key: u64) -> u64 {
    let mut rev_key: u64 = 0;

    for i in (0..ROUNDS).rev() {
        rev_key += (subkey(key, i) as u64) << (KEY_SIZE/ROUNDS * (ROUNDS - (i+1)));
    }

    rev_key
}

fn subkey(key: u64, idx: u8) -> u8 {
    (key >> (KEY_SIZE/ROUNDS * idx)) as u8
}

/*
    The round function: a psuedo-random number generator.
*/
fn prng(seed: u32, c: u8) -> u32 {
    let mut x: u64 = seed as u64;
    for _ in 0..4 {
        x = (7919*x + (c as u64)) % u32::MAX as u64;
    }

    x as u32
}

fn split(x: u64) -> (u32, u32) {
     ((x >> 32) as u32, x as u32)
}

fn combine((l, r): (u32, u32)) -> u64 {
    ((l as u64) << 32) + (r as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let data: u64 = 123456789;
        let key: u64 = 123;

        assert_eq!(data, decrypt(encrypt(data,key),key));
    }

    #[test]
    fn test_reverse_key() {
        let key: u64 = 0x0123456789abcdef;
        let rev_key: u64 = 0xefcdab8967452301;

        assert_eq!(rev_key, reverse_key(key));
    }
}