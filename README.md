# Encryption
I created an encryption algorithm that can encrypt an arbitrary amount of data with a 64 bit key. 

## How it works.
It uses a feinstel cipher where a PRNG is used as the round function. However, the feinstel cipher can only encrypt 64 bit blocks of data. 
I use the counter mode of operation to encrypt arbitrary amount of data while encrypting identical blocks differently.
