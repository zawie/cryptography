use std::fs::File;
use std::fs::OpenOptions;

use crypto::nonce::encrypt_stream;
use crypto::nonce::decrypt_stream;

fn main() {
    let key = 0x2ab56ef8dc104566;

    let input_path = "./resources/cat.webp";
    let encrypt_path = "./resources/encrypted_data";
    let output_path = "./resources/output.webp";

    let mut data_read = File::open(input_path).unwrap();

    let mut encrypted_write = OpenOptions::new()
        .write(true)
        .append(true)
        .open(encrypt_path)
        .unwrap();

    encrypt_stream(&mut data_read, &mut encrypted_write, key);

    let mut encrypted_read = File::open(encrypt_path).unwrap();

    let mut decrypted_write = OpenOptions::new()
        .write(true)
        .append(true)
        .open(output_path)
        .unwrap();

    decrypt_stream(&mut encrypted_read, &mut decrypted_write, key);

}