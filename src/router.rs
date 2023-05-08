use actix_web::{get, web::Path, HttpRequest, HttpResponse, Responder, Result};

#[get("/")]
async fn homepage(_request: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Slothy")
}

#[get("/water/{plant}")]
async fn get_water(path: Path<String>) -> Result<String> {
    let plant = path.into_inner();
    let level = 42.0;
    Ok(format!("Current water level of {plant} is {level}"))
}
