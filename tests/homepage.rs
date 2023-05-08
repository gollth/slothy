pub mod common;

use actix_web::{
    http::StatusCode,
    test::{call_service, init_service, read_body, TestRequest},
    web::Bytes,
    App,
};
use slothy::{server, Database};

#[sqlx::test]
async fn test_default_request_leads_to_homepage(db: Database) {
    common::setup();
    let app = init_service(App::new().configure(|c| server(db, c))).await;
    let request = TestRequest::default().to_request();

    let response = call_service(&app, request).await;
    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(Bytes::from_static(b"Slothy"), read_body(response).await);
}
