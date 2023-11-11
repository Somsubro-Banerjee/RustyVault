use crate::{
    api::{routes::data_models::NewVault, services::docs::docs},
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
/// ## Working ##
/// creating a new vault using `let vault = Node::new(body.name.clone());` <br>
/// declaring a vector of Nodes representing the list of replicas using: `let mut replicas: Vec<Node> = Vec::new();` <br>
/// Then used pattern matching to return the list of replicas or an error.
/// ```
/// #[post("/api/v1/vault/create")]
/// async fn new_vault(body: web::Json<NewVault>) -> impl Responder {
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
///            // Respond with a success message
///            HttpResponse::Created().json(replicas)
///        }
///        //in case of error respond with Internal Server Error.
///        Err(_) => HttpResponse::InternalServerError().json("Internal Server error"),
///    }
/// }
/// ```
#[post("/api/v1/vault/create")]
async fn new_vault(body: web::Json<NewVault>) -> impl Responder {
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
            // Respond with a success message
            HttpResponse::Created().json(replicas)
        }
        //in case of error respond with Internal Server Error.
        Err(_) => HttpResponse::InternalServerError().json("Internal Server error"),
    }
}
