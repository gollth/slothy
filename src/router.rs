use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get, put,
    web::{Data, Path},
    Error, HttpRequest, HttpResponse, Responder,
};

use crate::{
    types::{Humidity, Plant},
    AppState,
};

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
async fn get_plant(state: Data<AppState>, path: Path<String>) -> Result<HttpResponse, Error> {
    let name = path.into_inner();
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

#[get("/water/{plant}")]
async fn get_water(state: Data<AppState>, path: Path<String>) -> Result<HttpResponse, Error> {
    let plant = path.into_inner();
    let water = sqlx::query!(
        "SELECT humidity \
         FROM Water \
         INNER JOIN Plant ON Plant.id=Water.plant \
         WHERE UPPER(Plant.name)=UPPER(?) \
         ORDER BY stamp DESC;",
        plant
    )
    .fetch_optional(&state.db)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or(ErrorNotFound(format!("No such plant with name {plant}")))?
    .humidity;

    Ok(HttpResponse::Ok().json(water))
}

#[put("/water/{plant}/{humidity}")]
async fn put_water(
    state: Data<AppState>,
    path: Path<(String, Humidity)>,
) -> Result<HttpResponse, Error> {
    let (plant, humid) = path.into_inner();

    sqlx::query!(
        "INSERT INTO Water (plant, humidity) \
         VALUES            ((SELECT id FROM Plant WHERE UPPER(name)=UPPER(?)), ?)",
        plant,
        humid
    )
    .execute(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(_) => ErrorNotFound(format!("No such plant with name {plant}")),
        e => ErrorInternalServerError(e),
    })?;

    Ok(HttpResponse::Ok().finish())
}
