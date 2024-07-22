use actix_web::{App, HttpServer};
use server::{create_app_state, configure_services};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = create_app_state();

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(configure_services)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
