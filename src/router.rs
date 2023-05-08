use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get,
    web::{Data, Path},
    Error, HttpRequest, HttpResponse, Responder,
};

use crate::{types::Plant, AppState};

#[get("/")]
async fn homepage(_request: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Slothy")
}

#[get("/plant")]
async fn get_plants(state: Data<AppState>) -> Result<HttpResponse, Error> {
    let plants = sqlx::query_as!(Plant, "SELECT * FROM Plant")
        .fetch_all(&state.db)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(plants))
}

#[get("/plant/{name}")]
async fn get_plant(state: Data<AppState>, args: Path<String>) -> Result<HttpResponse, Error> {
    let name = args.into_inner();
    let plant = sqlx::query_as!(
        Plant,
        "SELECT * FROM Plant WHERE UPPER(name) = UPPER(?)",
        name
    )
    .fetch_optional(&state.db)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or(ErrorNotFound(format!("No such Plant with name {name}")))?;

    Ok(HttpResponse::Ok().json(plant))
}
