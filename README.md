# Obvious Note
Do not use this implementation in production. This is just a hobby repository.

# Feistel Cipher

## Usage

```cli
Usage: feistel [COMMAND]

Commands:
  encrypt  Encrypt data using Feistel cipher
  decrypt  Decrypt data using Feistel cipher
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Examples

```cli
cat sensitive.png | feistel encrypt --key 123 > encrypted.dat
cat encrypted.dat | feistel decrypt --key 123 > sensitive_decrypted.png
```

```cli
echo 'Hello, world!' | feistel encrypt | feistel decrypt 
Hello, world!
```

## About

This Feistel cipher uses a PRNG as the round function. However, the feistel cipher can only encrypt 64 bit blocks of data. I use the counter mode of operation to encrypt arbitrary amount of data while encrypting identical blocks differently.
