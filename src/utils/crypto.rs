use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use generic_array::typenum::U32;
use hex::{decode, encode};
use std::env;

const KEY_SIZE: usize = 32;

fn get_encryption_key() -> GenericArray<u8, U32> {
  let key_hex: String = env::var("ENCRYPTION_KEY")
    .expect("Chave de criptografia não encontrada no .env");
  let key_bytes: Vec<u8> =
    decode(key_hex).expect("Erro ao decodificar a chave de criptografia");

  let mut key_array: GenericArray<u8, U32> = GenericArray::default();
  key_array.copy_from_slice(&key_bytes[..KEY_SIZE]);
  key_array
}

pub fn encrypt_data(plaintext: &str) -> String {
  let key = get_encryption_key();
  let cipher = Aes256Gcm::new(&key);

  let nonce_hex = env::var("NONCE").expect("NONCE not set in .env");

  let nonce_bytes =
    hex::decode(nonce_hex).expect("Failed to decode nonce from hex");

  if nonce_bytes.len() != 12 {
    panic!(
      "Nonce must be 12 bytes long, but got {} bytes",
      nonce_bytes.len()
    );
  }

  let nonce: GenericArray<u8, _> = GenericArray::clone_from_slice(&nonce_bytes);

  let ciphertext = cipher
    .encrypt(&nonce, plaintext.as_bytes())
    .expect("Erro ao criptografar");

  encode(ciphertext)
}

pub fn decrypt_data(ciphertext: &str) -> String {
  let key = get_encryption_key();
  let cipher = Aes256Gcm::new(&key);

  let nonce_hex = env::var("NONCE").expect("NONCE not set in .env");

  let nonce = decode(nonce_hex).expect("Erro ao decodificar o nonce");
  let decoded_ciphertext = decode(ciphertext).expect("Erro ao decodificar hex");

  let plaintext = cipher
    .decrypt(Nonce::from_slice(&nonce), decoded_ciphertext.as_ref())
    .expect("Erro ao descriptografar");

  String::from_utf8(plaintext).expect("Erro ao converter para string")
}
