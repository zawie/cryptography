use crypto::countermode::encrypt_stream;
use crypto::countermode::decrypt_stream;

use crypto::fileio::open_read;
use crypto::fileio::open_write;

#[allow(unused_must_use)]
fn main() {
    let key = 0x2ab56ef8dc104566;

    let input_path = "./resources/cat.webp";
    let encrypt_path = "./resources/encrypted_data";
    let output_path = "./resources/output.webp";

    let mut data_read = open_read(input_path);
    let mut encrypted_write = open_write(encrypt_path);
    encrypt_stream(&mut data_read, &mut encrypted_write, key);

    let mut encrypted_read = open_read(encrypt_path);
    let mut decrypted_write = open_write(output_path);
    decrypt_stream(&mut encrypted_read, &mut decrypted_write, key);

}