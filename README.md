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

## Example

```cli
cat sensitive.png | feistel encrypt --key 123 > encrypted.dat
cat encrypted.dat | feistel decrypt --key 123 > sensitive_decrypted.png
```

## About

This Feistel cipher uses a PRNG as the round function. However, the feistel cipher can only encrypt 64 bit blocks of data. I use the counter mode of operation to encrypt arbitrary amount of data while encrypting identical blocks differently.
