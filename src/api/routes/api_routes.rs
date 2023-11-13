use crate::{
    api::{routes::{data_models::NewVault, state::AppState}, services::docs::docs},
    node::node::Node,
};
use actix_web::{get, post, web, HttpResponse, Responder};
/// This is the basic startup function that responds with a Startup message.
/// ```
/// #[get("/")]
/// async fn startup() -> impl Responder {
///    HttpResponse::Ok().body("Hello and welcome to RustyVault, please navigate to 'http://127.0.0.1:8080/api/v1/docs' to read the documentation on API usage")
/// }
/// ```
#[get("/")]
async fn startup() -> impl Responder {
    HttpResponse::Ok().body("Hello and welcome to RustyVault, please navigate to 'http://127.0.0.1:8080/api/v1/docs' to read the documentation on API usage")
}

/// This route call returns the minimal API docs needed for further development and usecases.
/// ```
/// #[get("/api/v1/docs")]
/// async fn api_docs() -> impl Responder {
///    let get_docs = docs();
///    HttpResponse::Ok().body(get_docs)
/// }
/// ```
#[get("/api/v1/docs")]
async fn api_docs() -> impl Responder {
    let get_docs = docs();
    HttpResponse::Ok().body(get_docs)
}

/// This route calls creates a new vault retuns the list of vaults in as per the replica count.
/// 
/// ## Working ##
/// creating a new vault using `let vault = Node::new(body.name.clone());` <br>
/// declaring a vector of Nodes representing the list of replicas using: `let mut replicas: Vec<Node> = Vec::new();` <br>
/// Then used pattern matching to return the list of replicas or an error.
/// ```
/// #[post("/api/v1/vault/create")]
/// async fn new_vault(body: web::Json<NewVault>, app_state: web::Data<AppState>) -> impl Responder {
///    // Attempt to create a new vault
///    let vault = Node::new(body.name.clone());
///    let mut replicas: Vec<Node> = Vec::new();
///    match vault {
///        Ok(vault) => {
///           // Replicate if replicas are requested
///            if body.replicas > 0 {
///                unsafe {
///                    replicas = vault.replicate(body.replicas);
///                }
///            }
///             // Add the new vault to the list
///             app_state.vaults.lock().unwrap().push(vault);
///            // Respond with a success message
///            HttpResponse::Created().json(replicas)
///        }
///        //in case of error respond with Internal Server Error.
///        Err(_) => HttpResponse::InternalServerError().json("Internal Server error"),
///    }
/// }
/// ```
#[post("/api/v1/vault/create")]
async fn new_vault(
    body: web::Json<NewVault>,
    app_state: web::Data<AppState>, // Access application state
) -> impl Responder {
    // Attempt to create a new vault
    let vault = Node::new(body.name.clone());
    let mut replicas: Vec<Node> = Vec::new();
    match vault {
        Ok(vault) => {
            // Replicate if replicas are requested
            if body.replicas > 0 {
                unsafe {
                    replicas = vault.replicate(body.replicas);
                }
            }
            // Add the new vault to the list
            app_state.vaults.lock().unwrap().push(vault);
            // Add the replicas to the list.
            for i in &replicas {
                app_state.replicas.lock().unwrap().push(i.clone());
            }
            // Respond with a success message
            HttpResponse::Created().json(replicas)
        }
        // in case of error respond with Internal Server Error.
        Err(_) => HttpResponse::InternalServerError().json("Internal Server error"),
    }
}

/// This route calls lists the total number of vaults present without the number of replicas.
/// 
/// ## Working ##
/// Since the AppState is a Mutex so we first try to lock the Mutex so that we can use it safely using `app_state.vaults.lock()`.
/// using pattern matcing we are trying to collect all the available vaults.
/// 
/// ```
/// #[get("/api/v1/vault/list")]
/// async fn list_vault(app_state: web::Data<AppState>) -> impl actix_web::Responder {
///    // Attempt to lock the mutex and handle potential errors
///    match app_state.vaults.lock() {
///        Ok(vaults) => {
///            // Successfully locked the mutex, respond with the list of vaults
///            let vault_list: Vec<Node> = vaults.clone(); // Clone the inner data
///            HttpResponse::Ok().json(vault_list)
///        }
///        Err(poisoned) => {
///            // Mutex is poisoned (another thread panicked while holding the lock)
///            HttpResponse::InternalServerError().json(format!("Mutex is poisoned: {:?}", poisoned))
///        }
///    }
/// }
/// ```
#[get("/api/v1/vault/list")]
async fn list_vault(app_state: web::Data<AppState>) -> impl Responder {
    // Attempt to lock the mutex and handle potential errors
    match app_state.vaults.lock() {
        Ok(vaults) => {
            // Successfully locked the mutex, respond with the list of vaults
            let vault_list: Vec<Node> = vaults.clone(); // Clone the inner data
            println!("{:#?}", vault_list.clone());
            HttpResponse::Ok().json(vault_list)
        }
        Err(poisoned) => {
            // Mutex is poisoned (another thread panicked while holding the lock)
            HttpResponse::InternalServerError().json(format!("Mutex is poisoned: {:?}", poisoned))
        }
    }
}

#[get("/api/v1/vault/replicas")]
async fn get_replicas(app_state: web::Data<AppState>) -> impl Responder {
    match app_state.replicas.lock() {
        Ok(replicas) => {
            // Successfully locked the mutex, respond with the list of vaults
            let replica_list: Vec<Node> = replicas.clone(); // Clone the inner data
            HttpResponse::Ok().json(replica_list)
        }
        Err(poisoned) => {
            // Mutex is poisoned (another thread panicked while holding the lock)
            HttpResponse::InternalServerError().json(format!("Mutex is poisoned: {:?}", poisoned))
        }
    }
}