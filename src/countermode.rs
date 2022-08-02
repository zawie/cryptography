use std::io::{Read, Write};

use crate::feistel::cipher;
use crate::feistel::reverse_key;
use crate::util::word_to_bytes;

pub fn encrypt_stream(src: &mut dyn Read, out: &mut dyn Write, key: u64) -> Result<(), Box<dyn std::error::Error>> {
    let mut bytes = src.bytes();
    
    //Generate nonce and write it to output
    let nonce: u64 = 0xa1b2034cdef10000;
    out.write(&word_to_bytes(cipher(nonce, key))); 

    let mut counter: u64 = 0;
    let mut buff: u64 = 0;

    //Encrypt file
    loop {
        if counter % 8 == 0 && counter > 0 {
            out.write(&word_to_bytes(cipher(nonce + counter + 1, key) ^ buff));
            buff = 0;
        }

        if let Some(byte_result) = bytes.next() {
            let byte = byte_result?;
            buff += (byte as u64) << (8*(7-(counter % 8)));
            counter += 1;
        } else {
            break;
        }
    }

    //Add padding
    let pad_count: u8 = 8 - (counter as u8 % 8);
    for _ in 0..pad_count as u64 {
        buff += (pad_count as u64) << (8*(7-(counter % 8)));
        counter += 1;
    }
    out.write(&word_to_bytes(cipher(nonce + counter + 1, key) ^ buff));

    Ok(())
}

pub fn decrypt_stream(src: &mut dyn Read, out: &mut dyn Write, key: u64) -> Result<(), Box<dyn std::error::Error>> {
    let mut bytes = src.bytes();
    
    //Decrypt nonce
    let mut encrypted_nonce: u64 = 0;
    for i in 0..8 {
        if let Some(byte_result) = bytes.next() {
            let byte = byte_result?;
            encrypted_nonce += (byte as u64) << (8*(7-i));
        } else {
            panic!("Nonce not found!");
        }
    }
    let nonce: u64 = cipher(encrypted_nonce, reverse_key(key));

    let mut counter: u64 = 0;
    let mut buff: u64 = 0;

    //Decrypt file
    loop {
        
        if let Some(byte_result) = bytes.next() {
            if counter > 0 {
                //Write last word
                out.write(&word_to_bytes(cipher(nonce + counter + 1, key) ^ buff));

                //Reset buffer
                buff = 0;
            }
            
            //Write first byte into buff
            let byte = byte_result?;
            buff += (byte as u64) << (8*(7-(counter % 8)));
            counter += 1;

        } else { 
            //No more bytes to read.
            //Decrypt last word
            let decrypted_buff = cipher(nonce + counter + 1, key) ^ buff;

            //Get padding
            let pad_count: u8 = (decrypted_buff & 0x00000000000000ff) as u8;

            //Write last word, excluding padding
            out.write(&word_to_bytes(decrypted_buff)[0..(8-pad_count as usize)]);

            //Finish decrypting
            break;
        }

        //Read in remaining bytes
        for _ in 1..8 {
            if let Some(byte_result) = bytes.next() {
                let byte = byte_result?;
                buff += (byte as u64) << (8*(7-(counter % 8)));
                counter += 1;
            } else {
                panic!("Unexpected byte count!");
            }
        }
    }

    Ok(())
}
