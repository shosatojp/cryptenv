use std::iter::FromIterator;

use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;
const IV_LENGTH: usize = 16;

pub fn encrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    let _key = openssl::sha::sha256(key);
    let iv: [u8; IV_LENGTH] = rand::random();
    let cipher = Aes256Cbc::new_from_slices(&_key, &iv).unwrap();
    let enc = cipher.encrypt_vec(data);

    // concat iv and enc
    let mut v: Vec<u8> = Vec::from_iter(iv);
    v.extend(enc);
    v
}

pub fn decrypt(key: &[u8], enc: &[u8]) -> Vec<u8> {
    let _key = openssl::sha::sha256(key);
    let (iv, data) = enc.split_at(IV_LENGTH);
    let cipher = Aes256Cbc::new_from_slices(&_key, iv).unwrap();
    cipher.decrypt_vec(data).unwrap()
}

#[test]
pub fn test_encrypt_decrypt() {
    let key = "hoge".as_bytes();
    let data = "this is data".as_bytes();
    let enc = encrypt(key, data);
    let dec = decrypt(key, &enc);
    assert_eq!(data, dec);
}
