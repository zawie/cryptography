use crypto::encrypt;
use crypto::decrypt;

fn main() {
    let data = 9113372034454775809;
    let key = 984530987;

    let cipher = encrypt(data,key);
    let decrypted_cipher = decrypt(cipher, key);

    println!("data\t{:x}\ncipher\t{:x}\ndecrypt\t{:x}", data, cipher, decrypted_cipher)
}
