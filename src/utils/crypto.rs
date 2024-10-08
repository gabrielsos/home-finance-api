use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, Nonce};
use aes_gcm::aead::generic_array::GenericArray;
use dotenv::dotenv;
use hex::{encode, decode};
use std::env;
use generic_array::typenum::{UInt, UTerm, B0, B1, U32};

const NONCE_SIZE: usize = 12;
const KEY_SIZE: usize = 32;

fn get_encryption_key() -> GenericArray<u8, U32> {
    dotenv().ok();
    let key_hex = env::var("ENCRYPTION_KEY").expect("Chave de criptografia não encontrada no .env");
    let key_bytes = decode(key_hex).expect("Erro ao decodificar a chave de criptografia");

    let mut key_array: GenericArray<u8, U32> = GenericArray::default();
    key_array.copy_from_slice(&key_bytes[..KEY_SIZE]);
    key_array
}

pub fn generate_nonce() -> aes_gcm::aead::generic_array::GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>> {
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // Gera um nonce aleatório
    nonce
}

pub fn encrypt_data(plaintext: &str, nonce: &aes_gcm::aead::generic_array::GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>) -> String {
    let key = get_encryption_key();
    let cipher = Aes256Gcm::new(&key);
    
    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .expect("Erro ao criptografar");
    
    encode(ciphertext)
}

pub fn decrypt_data(ciphertext: &str, nonce_hex: &str) -> String {
    let key = get_encryption_key();
    let cipher = Aes256Gcm::new(&key);
    
    let nonce = decode(nonce_hex).expect("Erro ao decodificar o nonce");
    let decoded_ciphertext = decode(ciphertext).expect("Erro ao decodificar hex");
    
    let plaintext = cipher
        .decrypt(Nonce::from_slice(&nonce), decoded_ciphertext.as_ref())
        .expect("Erro ao descriptografar");

    String::from_utf8(plaintext).expect("Erro ao converter para string")
}
