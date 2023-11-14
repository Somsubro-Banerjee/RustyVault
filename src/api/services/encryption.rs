use sodiumoxide::crypto::{
    aead::xchacha20poly1305_ietf,
    secretbox::{self, Key},
};

pub struct Encryption {}

impl Encryption {
    fn get_key() -> Key {
        secretbox::gen_key()
    }

    pub async fn encrypt(data: &[u8], key: &secretbox::Key) -> Vec<u8> {
        let nonce = secretbox::gen_nonce();
        let ciphertext = secretbox::seal(data, &nonce, key);
        let mut result = Vec::new();
        result.extend_from_slice(&nonce.0);
        result.extend_from_slice(&ciphertext);
        result
    }
    pub async fn decrypt(data: &[u8], key: &secretbox::Key) -> Option<Vec<u8>> {
        if data.len() < secretbox::NONCEBYTES {
            return None;
        }
        let nonce = secretbox::Nonce::from_slice(&data[..secretbox::NONCEBYTES])?;
        let ciphertext = &data[secretbox::NONCEBYTES..];
        secretbox::open(ciphertext, &nonce, key).ok()
    }
}
