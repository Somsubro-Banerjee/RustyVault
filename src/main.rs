pub mod api;
pub mod node;
use crate::api::routes::api_routes::{api_docs, new_vault, startup};
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("RustyVault is listening on 'http://127.0.0.1:8080'");
    HttpServer::new(|| {
        App::new()
            .service(startup)
            .service(api_docs)
            .service(new_vault)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
