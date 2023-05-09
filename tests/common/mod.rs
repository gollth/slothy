use actix_web::{dev::ServiceResponse, http::StatusCode};

// Keep these consistent with `fixtures/plants.sql`
pub const ID_MINZE: i64 = 0;
pub const ID_SCHNITTLAUCH: i64 = 1;
pub const ID_PAPRIKA: i64 = 2;

pub fn setup() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Debug)
        .try_init();
}
fn assert_status(code: StatusCode, response: &ServiceResponse) {
    assert_eq!(code, response.status());
}

pub fn assert_status_ok(response: &ServiceResponse) {
    assert_status(StatusCode::OK, response);
}

pub fn assert_status_not_found(response: &ServiceResponse) {
    assert_status(StatusCode::NOT_FOUND, response);
}

pub fn assert_status_bad_request(response: &ServiceResponse) {
    assert_status(StatusCode::BAD_REQUEST, response);
}
