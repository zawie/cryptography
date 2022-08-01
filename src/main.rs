use std::fs::File;
use std::fs::OpenOptions;

use crypto::nonce::encrypt_stream;
use crypto::nonce::decrypt_stream;

fn main() {
    let key = 0x2ab56ef8dc104566;

    let mut data_read = File::open("./resources/data.txt").unwrap();

    let mut encrypted_write = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./resources/encrypted_data.txt")
        .unwrap();

    encrypt_stream(&mut data_read, &mut encrypted_write, key);

    let mut encrypted_read = File::open("./resources/encrypted_data.txt").unwrap();

    let mut decrypted_write = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./resources/decrypted_data.txt")
        .unwrap();

    decrypt_stream(&mut encrypted_read, &mut decrypted_write, key);

}