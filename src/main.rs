use crypto::encrypt;
use crypto::decrypt;

fn main() {
    let data = 1234567890;
    let key = 3;

    let cipher = encrypt(data,key);
    let decrypted_cipher = decrypt(cipher, key);

    println!("original data\t{}\nencrypted data\t{:x}\nuncrypted data\t{}", data, cipher, decrypted_cipher)
}
