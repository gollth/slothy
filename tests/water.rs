use actix_web::{
    test::{call_service, init_service, read_body_json, TestRequest},
    App,
};
use chrono::{SubsecRound, Utc};
use slothy::{server, types::Water, Database};

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
    let request = TestRequest::get().uri("/water/minze").to_request();

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

#[sqlx::test(fixtures("plants"))]
async fn put_water_for_plant_adds_humidity(db: Database) {
    common::setup();
    let humidity = 0.88;
    let plant = "minze";
    let plant_id = sqlx::query!("SELECT * FROM Plant WHERE UPPER(name)=UPPER(?);", plant)
        .fetch_one(&db)
        .await
        .unwrap()
        .id;
    let app = init_service(App::new().configure(|c| server(db.clone(), c))).await;
    let request = TestRequest::put()
        .uri(&format!("/water/{plant}/{humidity}"))
        .to_request();

    let response = call_service(&app, request).await;
    common::assert_status_ok(&response);

    let start = Utc::now().naive_local().trunc_subsecs(0);
    let measurement = sqlx::query_as!(
        Water,
        "SELECT Water.* FROM Water \
         INNER JOIN Plant ON Plant.id=Water.plant \
         WHERE UPPER(Plant.name)=UPPER(?) \
         AND humidity=?;",
        plant,
        humidity
    )
    .fetch_one(&db)
    .await
    .unwrap();

    assert_eq!(measurement.plant, plant_id);
    assert_eq!(measurement.stamp, start);
}

#[sqlx::test(fixtures("plants"))]
async fn put_water_for_plants_404s_if_plant_does_not_exist(db: Database) {
    common::setup();
    let app = init_service(App::new().configure(|c| server(db, c))).await;
    let request = TestRequest::put()
        .uri("/water/unknown_plant/42")
        .to_request();

    let response = call_service(&app, request).await;
    common::assert_status_not_found(&response);
}
