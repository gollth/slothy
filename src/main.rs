use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(slothy::server))
        .bind(("localhost", 51074))?
        .run()
        .await
}
