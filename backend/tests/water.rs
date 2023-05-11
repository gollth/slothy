use actix_web::{
    test::{call_service, init_service, read_body_json, TestRequest},
    App,
};
use chrono::{SubsecRound, Utc};
use slothy::{
    server,
    types::{Measurement, Observation, Plant},
    Database,
};

pub mod common;

#[sqlx::test(fixtures("plants", "water"))]
async fn get_water_for_plant_404s_if_no_such_plant_exist(db: Database) {
    common::setup();
    let app = init_service(App::new().configure(|c| server(db, c))).await;
    let request = TestRequest::get().uri("/water/unknown_plant").to_request();

    let response = call_service(&app, request).await;

    common::assert_status_not_found(&response);
}

#[sqlx::test(fixtures("plants", "water"))]
async fn get_water_for_plant_yields_latest_humidity(db: Database) {
    common::setup();
    let app = init_service(App::new().configure(|c| server(db, c))).await;
    let request = TestRequest::get().uri("/water/0/0").to_request();

    let response = call_service(&app, request).await;

    common::assert_status_ok(&response);

    let water: f32 = read_body_json(response).await;
    assert_eq!(0.42, water);
}

#[sqlx::test(fixtures("plants", "water"))]
async fn get_water_for_plant_404s_if_no_measurement_ever_taken_for_plant(db: Database) {
    common::setup();
    let app = init_service(App::new().configure(|c| server(db, c))).await;
    let request = TestRequest::get().uri("/water/paprika").to_request();

    let response = call_service(&app, request).await;
    common::assert_status_not_found(&response);
}

#[sqlx::test(fixtures("plants", "water"))]
async fn put_water_adds_humidity(db: Database) {
    common::setup();
    let humidity = 0.88;
    let iot = 0;
    let sensor = 1;

    let app = init_service(App::new().configure(|c| server(db.clone(), c))).await;
    let request = TestRequest::put()
        .uri(&format!("/water"))
        .set_json(Measurement {
            id: iot,
            sensor,
            humidity,
        })
        .to_request();

    let response = call_service(&app, request).await;
    common::assert_status_ok(&response);

    let plant = sqlx::query_as!(
        Plant,
        "SELECT * FROM Plant \
         WHERE iot=? AND sensor=?",
        iot,
        sensor
    )
    .fetch_one(&db)
    .await
    .unwrap();

    let start = Utc::now().naive_local().trunc_subsecs(0);
    let measurement = sqlx::query_as!(
        Observation,
        "SELECT * FROM Observation \
         WHERE plant=? \
         AND humidity=?;",
        plant.name,
        humidity
    )
    .fetch_one(&db)
    .await
    .unwrap();

    assert_eq!(measurement.plant, plant.name);
    assert_eq!(measurement.stamp, start);
    assert_eq!(measurement.humidity, humidity);
}

#[sqlx::test(fixtures("plants"))]
async fn put_water_404s_if_plant_does_not_exist(db: Database) {
    common::setup();
    let app = init_service(App::new().configure(|c| server(db, c))).await;
    let request = TestRequest::put()
        .uri("/water")
        .set_json(Measurement {
            id: 0,
            sensor: 17,
            humidity: 0.123,
        })
        .to_request();

    let response = call_service(&app, request).await;
    common::assert_status_not_found(&response);
}

#[sqlx::test(fixtures("plants"))]
async fn put_water_ignores_nan_values(db: Database) {
    common::setup();
    let id = 0;
    let sensor = 0;
    let app = init_service(App::new().configure(|c| server(db.clone(), c))).await;
    let request = TestRequest::put()
        .uri("/water")
        .set_json(Measurement {
            id,
            sensor,
            humidity: f64::NAN,
        })
        .to_request();

    let response = call_service(&app, request).await;
    common::assert_status_bad_request(&response);
}
