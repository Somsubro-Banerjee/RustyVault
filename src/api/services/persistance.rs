use crate::api::routes::state::AppState;
use sodiumoxide::crypto::aead::{xchacha20poly1305_ietf, Key};
use sodiumoxide::crypto::secretbox;
use std::fs::File;
use std::io::{self, Read};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

use super::encryption::Encryption;

const PERSISTANCE_STORAGE: &str = "src/vaults.txt";

pub struct StoragePersistance {}

impl StoragePersistance {
    // Serialize AppState to bytes
    fn serialize_state(state: &AppState) -> Vec<u8> {
        serde_json::to_vec(state).unwrap()
    }

    // Deserialize bytes to AppState
    fn deserialize_state(data: &[u8]) -> Option<AppState> {
        serde_json::from_slice(data).ok()
    }

    //
    pub async fn save_data_to_file_encrypted(
        state: &AppState,
        key: &secretbox::Key,
    ) -> std::io::Result<()> {
        let serialized_data = Self::serialize_state(state);
        let encrypted_data = Encryption::encrypt(&serialized_data, key);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(PERSISTANCE_STORAGE)
            .await?;

        file.write_all(&encrypted_data.await).await?;

        Ok(())
    }

    pub async fn load_data_from_file_encrypted(key: &secretbox::Key) -> Option<AppState> {
        match File::open(PERSISTANCE_STORAGE) {
            Ok(mut file) => {
                let mut encrypted_data = Vec::new();
                if let Err(err) = file.read_to_end(&mut encrypted_data) {
                    eprintln!("Error reading file: {}", err);
                    return None;
                }

                match Encryption::decrypt(&encrypted_data, key).await {
                    Some(serialized_data) => {
                        Some(Self::deserialize_state(&serialized_data).unwrap())
                    }
                    None => None,
                }
            }
            Err(err) => {
                eprintln!("Error opening file: {}", err);
                None
            }
        }
    }

    pub fn save_encrypted_key_to_file(
        key: &secretbox::Key,
        master_key: &Key,
        filename: &str,
    ) -> Result<(), io::Error> {
        let nonce = xchacha20poly1305_ietf::gen_nonce();
        let encrypted_key = xchacha20poly1305_ietf::seal(key.as_ref(), None, &nonce, master_key);

        std::fs::write(filename, &encrypted_key)
    }

    pub fn load_encrypted_key_from_file(
        master_key: &Key,
        filename: &str,
    ) -> Result<secretbox::Key, io::Error> {
        if let Ok(encrypted_key) = std::fs::read(filename) {
            let nonce = xchacha20poly1305_ietf::gen_nonce();
            xchacha20poly1305_ietf::open(&encrypted_key, None, &nonce, &master_key)
                .map(|decrypted_key| secretbox::Key(decrypted_key.try_into().unwrap()))
                .map_err(|_| io::ErrorKind::InvalidData.into())
        } else {
            Err(io::ErrorKind::NotFound.into())
        }
    }
}
