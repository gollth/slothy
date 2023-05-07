use actix_web::{get, HttpRequest, HttpResponse, Responder};

#[get("/")]
async fn homepage(_request: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Slothy")
}
