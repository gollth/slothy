use actix_web::{
    test::{call_service, init_service, read_body_json, TestRequest},
    App,
};
use slothy::{server, types::Plant, Database};

pub mod common;

#[sqlx::test(fixtures("plants"))]
async fn get_plants_index_yields_list(db: Database) {
    common::setup();
    let app = init_service(App::new().configure(|c| server(db, c))).await;
    let request = TestRequest::get().uri("/plant/0").to_request();

    let response = call_service(&app, request).await;

    common::assert_status_ok(&response);
    let plants: Vec<Plant> = read_body_json(response).await;
    assert_eq!(
        vec![
            Plant::new(0, 0, "Minze"),
            Plant::new(0, 1, "Schnittlauch"),
            Plant::new(0, 2, "Paprika")
        ],
        plants
    );
}

#[sqlx::test]
async fn get_plants_index_yields_empty_list_if_none_ever_created(db: Database) {
    common::setup();
    let app = init_service(App::new().configure(|c| server(db, c))).await;
    let request = TestRequest::get().uri("/plant/0").to_request();

    let response = call_service(&app, request).await;

    common::assert_status_ok(&response);
    let plants: Vec<Plant> = read_body_json(response).await;
    assert_eq!(plants, vec![])
}

#[sqlx::test(fixtures("plants"))]
async fn get_plant_route_looksup_name_by_iot_and_sensor(db: Database) {
    common::setup();
    let app = init_service(App::new().configure(|c| server(db, c))).await;
    for (iot, sensor, name) in [(0, 0, "Minze"), (0, 1, "Schnittlauch"), (0, 2, "Paprika")] {
        let request = TestRequest::get()
            .uri(&format!("/plant/{iot}/{sensor}"))
            .to_request();

        let response = call_service(&app, request).await;

        common::assert_status_ok(&response);
        let plant: Plant = read_body_json(response).await;

        assert_eq!(plant, Plant::new(iot, sensor, name),);
    }
}

#[sqlx::test(fixtures("plants"))]
async fn get_plant_route_404s_if_name_not_given(db: Database) {
    common::setup();
    let app = init_service(App::new().configure(|c| server(db, c))).await;
    let request = TestRequest::get().uri("/plant/unknown_plant").to_request();

    let response = call_service(&app, request).await;

    common::assert_status_not_found(&response);
}
