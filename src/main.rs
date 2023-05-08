use actix_web::{middleware::Logger, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use dotenv_codegen::dotenv;
use log::info;
use sqlx::SqlitePool;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::builder().format_timestamp(None).init();

    let url = dotenv!("DATABASE_URL");
    let db = SqlitePool::connect(url).await?;
    info!("Connected to database {url}");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(|c| slothy::server(db.clone(), c))
    })
    .bind(("localhost", 51074))?
    .run()
    .await?;
    Ok(())
}
