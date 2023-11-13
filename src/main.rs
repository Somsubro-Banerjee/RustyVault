pub mod api;
pub mod node;
use crate::api::routes::api_routes::{api_docs, new_vault, startup, list_vault, get_replicas};
use crate::api::routes::state::AppState;
use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState::new());

    println!("RustyVault is listening on 'http://127.0.0.1:8080'");
    HttpServer::new(move || {
        App::new()
        .app_data(app_state.clone())
            .service(startup)
            .service(api_docs)
            .service(new_vault)
            .service(list_vault)
            .service(get_replicas)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
