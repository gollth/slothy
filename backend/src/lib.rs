mod router;
pub mod types;

use crate::router::homepage;

use actix_web::web::{Data, ServiceConfig};
use router::{get_plant, get_plants, get_water, post_water};
use sqlx::{Pool, Sqlite};

pub type Database = Pool<Sqlite>;

struct AppState {
    pub db: Database,
}

pub fn server(db: Database, config: &mut ServiceConfig) {
    config
        .app_data(Data::new(AppState { db }))
        .service(homepage)
        .service(get_plant)
        .service(get_plants)
        .service(get_water)
        .service(post_water);
}
