use actix_web::{
    test::{call_service, init_service, read_body_json, TestRequest},
    App,
};
use slothy::{server, Database};

pub mod common;

#[sqlx::test(fixtures("plants", "water"))]
async fn get_water_for_plant_404s_if_no_such_plant_exist(db: Database) {
    common::setup();
    let app = init_service(App::new().configure(|c| server(db, c))).await;
    let request = TestRequest::default()
        .uri("/water/unknown_plant")
        .to_request();

    let response = call_service(&app, request).await;

    common::assert_status_not_found(&response);
}

#[sqlx::test(fixtures("plants", "water"))]
async fn get_water_for_plant_yields_latest_humidity(db: Database) {
    common::setup();
    let app = init_service(App::new().configure(|c| server(db, c))).await;
    let request = TestRequest::default().uri("/water/minze").to_request();

    let response = call_service(&app, request).await;

    common::assert_status_ok(&response);

    let water: f32 = read_body_json(response).await;
    assert_eq!(0.42, water);
}

#[sqlx::test(fixtures("plants", "water"))]
async fn get_water_for_plant_404s_if_no_measurement_ever_taken_for_plant(db: Database) {
    common::setup();
    let app = init_service(App::new().configure(|c| server(db, c))).await;
    let request = TestRequest::default().uri("/water/paprika").to_request();

    let response = call_service(&app, request).await;
    common::assert_status_not_found(&response);
}
