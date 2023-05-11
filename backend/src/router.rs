use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get, put,
    web::{Data, Json, Path},
    Error, HttpRequest, HttpResponse, Responder,
};

use crate::{
    types::{Measurement, Observation, Plant},
    AppState,
};

#[get("/")]
async fn homepage(_request: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Slothy")
}

#[get("/plant/{iot}")]
async fn get_plants(state: Data<AppState>, path: Path<i64>) -> Result<HttpResponse, Error> {
    let iot = path.into_inner();
    let plants = sqlx::query_as!(Plant, "SELECT * FROM Plant WHERE iot=?", iot)
        .fetch_all(&state.db)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(plants))
}

#[get("/plant/{iot}/{sensor}")]
async fn get_plant(state: Data<AppState>, path: Path<(i64, i64)>) -> Result<HttpResponse, Error> {
    let (iot, sensor) = path.into_inner();
    let plant = sqlx::query_as!(
        Plant,
        "SELECT * FROM Plant WHERE iot = ? AND sensor = ?",
        iot,
        sensor
    )
    .fetch_optional(&state.db)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or(ErrorNotFound(format!(
        "No plant configured for IoT device #{iot} and sensor #{sensor}"
    )))?;

    Ok(HttpResponse::Ok().json(plant))
}

#[get("/water/{iot}/{sensor}")]
async fn get_water(state: Data<AppState>, path: Path<(i64, i64)>) -> Result<HttpResponse, Error> {
    let (iot, sensor) = path.into_inner();
    let observation = sqlx::query_as!(
        Observation,
        "SELECT * FROM Observation \
         WHERE plant = ( \
           SELECT name FROM PLANT \
           WHERE iot = ? AND sensor = ? \
         ) \
         ORDER BY stamp DESC;",
        iot,
        sensor
    )
    .fetch_optional(&state.db)
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or(ErrorNotFound(format!(
        "No plant configured for IoT device #{iot} and sensor #{sensor}"
    )))?
    .humidity;

    Ok(HttpResponse::Ok().json(observation))
}

#[put("/water")]
async fn put_water(
    state: Data<AppState>,
    payload: Json<Measurement>,
) -> Result<HttpResponse, Error> {
    sqlx::query!(
        "INSERT INTO Observation (plant, humidity) \
         VALUES ((SELECT name FROM Plant WHERE iot = ? AND sensor = ?), ?)",
        payload.id,
        payload.sensor,
        payload.humidity
    )
    .execute(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(_) => ErrorNotFound(format!(
            "No plant configured for IoT device #{} and sensor #{}",
            payload.id, payload.sensor
        )),
        e => ErrorInternalServerError(e),
    })?;

    Ok(HttpResponse::Ok().finish())
}
