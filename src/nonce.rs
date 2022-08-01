use std::io::{Read, Write};

use crate::feisel::cipher;
use crate::feisel::reverse_key;

pub fn encrypt_stream(src: &mut dyn Read, out: &mut dyn Write, key: u64) -> Result<(), Box<dyn std::error::Error>> {
    use_stream(src, out, key)
}

pub fn decrypt_stream(src: &mut dyn Read, out: &mut dyn Write, key: u64) -> Result<(), Box<dyn std::error::Error>> {
    use_stream(src, out, key)
}

fn use_stream(src: &mut dyn Read, out: &mut dyn Write, key: u64) -> Result<(), Box<dyn std::error::Error>> {
    let mut bytes = src.bytes();
    
    let nonce: u64 = 0xa1b2034cdef10000;

    let mut counter: u64 = 0;
    let mut buff: u64 = 0;

    loop {
        if counter % 8 == 0 && counter > 0 {
            out.write(&word_to_bytes(cipher(nonce + counter, key) ^ buff));
            buff = 0;
        }

        if let Some(byte_result) = bytes.next() {
            let byte = byte_result?;
            buff += (byte as u64) << (8*(7-(counter % 8)));
            counter += 1;
        } else {
            if counter % 8 > 0 {
                out.write(&word_to_bytes(cipher(nonce + counter, key) ^ buff));
            }
            break;
        }

    }

    Ok(())
}

fn word_to_bytes(word: u64) -> [u8; 8] {
    let mut arr: [u8; 8] = [0,0,0,0,0,0,0,0];
    for i in 1..=8 {
        arr[i-1] = (word >> (8 - i)*8) as u8
    }

    arr
}