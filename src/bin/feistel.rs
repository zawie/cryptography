use crypto::countermode::encrypt_stream;
use crypto::countermode::decrypt_stream;

use clap::{Arg, Command};
use std::io::{self, Read, Write};

fn main() {
    // Define the CLI structure
    let matches = Command::new("feistel")
        .version("1.0")
        .author("Adam Zawierucha")
        .about("Feistel Cipher CLI using stdin and stdout")
        .subcommand(
            Command::new("encrypt")
                .about("Encrypt data using Feistel cipher")
                .arg(Arg::new("key")
                    .short('k')
                    .long("key")
                    .value_name("NUMBER")
                    .help("Encryption key (optional, default is 12345)")
                    .default_value("1234567890")
                    .value_parser(clap::value_parser!(u64))  
            )
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypt data using Feistel cipher")
                .arg(Arg::new("key")
                    .short('k')
                    .long("key")
                    .value_name("NUMBER")
                    .help("Decryption key (optional, default is 12345)")
                    .default_value("1234567890")
                    .value_parser(clap::value_parser!(u64))  
           )
        )
        .get_matches();

    let mut stdin: Box<dyn Read> = Box::new(io::stdin());
    let mut stdout: Box<dyn Write> = Box::new(io::stdout());

    if let Some(sub_matches) = matches.subcommand_matches("encrypt") {
        let key = sub_matches.get_one::<u64>("key").unwrap(); // Since default_value guarantees presence

        let _ = encrypt_stream(&mut stdin, &mut stdout, *key);
    } else if let Some(sub_matches) = matches.subcommand_matches("decrypt") {
        let key = sub_matches.get_one::<u64>("key").unwrap(); // Since default_value guarantees presence

        let _ = decrypt_stream(&mut stdin, &mut stdout, *key);
    }
}
