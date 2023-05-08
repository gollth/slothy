use actix_web::{dev::ServiceResponse, http::StatusCode};

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
