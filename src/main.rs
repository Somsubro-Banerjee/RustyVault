pub mod api;
pub mod node;
use crate::api::routes::api_routes::{api_docs, get_replicas, list_vault, new_vault, startup};
use crate::api::routes::state::AppState;
use crate::api::services::persistance::StoragePersistance;
use actix_web::{web, App, HttpServer};
use sodiumoxide::crypto::{aead, secretbox};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = sodiumoxide::init();
    let master_key = aead::gen_key();

    if let Some(key) = load_or_generate_key(&master_key).await {
        run_server(&key).await;
    } else {
        eprintln!("Failed to load or generate key");
    }

    Ok(())
}

async fn load_or_generate_key(master_key: &aead::Key) -> Option<secretbox::Key> {
    match StoragePersistance::load_encrypted_key_from_file(master_key, "encrypted_secret.key") {
        Ok(key) => {
            println!("Using the loaded key for decryption: {:?}", key);
            Some(key)
        }
        Err(err) => {
            eprintln!("Failed to load or decrypt key: {}", err);
            generate_and_save_key(master_key).await
        }
    }
}

async fn generate_and_save_key(master_key: &aead::Key) -> Option<secretbox::Key> {
    let key = secretbox::gen_key();
    println!("Generated a new key: {:?}", key);

    if let Err(err) = StoragePersistance::save_encrypted_key_to_file(&key, master_key, "encrypted_secret.key") {
        eprintln!("Failed to save encrypted key: {}", err);
        return None;
    }

    println!("Encrypted key saved successfully.");
    Some(key)
}

async fn run_server(key: &secretbox::Key) {
    let existing_data = StoragePersistance::load_data_from_file_encrypted(key)
        .await
        .unwrap_or_else(|| AppState {
            vaults: Vec::new().into(),
            replicas: Vec::new().into(),
        });

    let app_state = web::Data::new(AppState::new(existing_data.vaults));
    let save_state = app_state.clone();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(startup)
            .service(api_docs)
            .service(new_vault)
            .service(list_vault)
            .service(get_replicas)
    })
    .shutdown_timeout(5)
    .bind(("127.0.0.1", 8080));

    match server {
        Ok(server) => {
            println!("RustyVault is listening on 'http://127.0.0.1:8080'");
            let runner = server.run();

            let save = StoragePersistance::save_data_to_file_encrypted(&save_state, key);
            match save.await {
                Ok(_) => println!("Data saved successfully"),
                Err(_) => println!("Unable to write to file"),
            }

            runner.await.unwrap();
        }
        Err(err) => {
            eprintln!("Failed to bind server: {}", err);
        }
    }
}
