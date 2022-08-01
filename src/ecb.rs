use std::io::{Read, Write};

use crate::feistel::cipher;
use crate::feistel::reverse_key;
use crate::util::word_to_bytes;

pub fn encrypt_stream(src: &mut dyn Read, out: &mut dyn Write, key: u64) -> Result<(), Box<dyn std::error::Error>> {
    use_stream(src, out, key)
}

pub fn decrypt_stream(src: &mut dyn Read, out: &mut dyn Write, key: u64) -> Result<(), Box<dyn std::error::Error>> {
    use_stream(src, out, reverse_key(key))
}

fn use_stream(src: &mut dyn Read, out: &mut dyn Write, key: u64) -> Result<(), Box<dyn std::error::Error>> {
    let mut bytes = src.bytes();
    
    let mut buff: u64 = 0;
    let mut buff_usage: u8 = 0;

    loop {
        if buff_usage == 8 {
            buff_usage = 0;
            out.write(&word_to_bytes(cipher(buff, key)));
            buff = 0;
        }

        if let Some(byte_result) = bytes.next() {
            let byte = byte_result?;
            buff_usage += 1;
            buff += (byte as u64) << (8*(8-buff_usage));
        } else {
            if buff_usage > 0 {
                out.write(&word_to_bytes(cipher(buff, key)));
            }
            break;
        }

    }

    Ok(())
}