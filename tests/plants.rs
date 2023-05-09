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
    let request = TestRequest::default().uri("/plant").to_request();

    let response = call_service(&app, request).await;

    common::assert_status_ok(&response);
    let plants: Vec<Plant> = read_body_json(response).await;
    assert_eq!(
        vec![
            Plant::new(0, "Minze"),
            Plant::new(1, "Schnittlauch"),
            Plant::new(2, "Paprika")
        ],
        plants
    );
}

#[sqlx::test]
async fn get_plants_index_yields_empty_list_if_none_ever_created(db: Database) {
    common::setup();
    let app = init_service(App::new().configure(|c| server(db, c))).await;
    let request = TestRequest::default().uri("/plant").to_request();

    let response = call_service(&app, request).await;

    common::assert_status_ok(&response);
    let plants: Vec<Plant> = read_body_json(response).await;
    assert_eq!(plants, vec![])
}

#[sqlx::test(fixtures("plants"))]
async fn get_plant_route_looksup_id_by_name(db: Database) {
    common::setup();
    let app = init_service(App::new().configure(|c| server(db, c))).await;
    let request = TestRequest::default().uri("/plant/minze").to_request();

    let response = call_service(&app, request).await;

    common::assert_status_ok(&response);
    let plant: Plant = read_body_json(response).await;

    assert_eq!(plant, Plant::new(0, "Minze"));
}

#[sqlx::test(fixtures("plants"))]
async fn get_plant_route_404s_if_name_not_given(db: Database) {
    common::setup();
    let app = init_service(App::new().configure(|c| server(db, c))).await;
    let request = TestRequest::default()
        .uri("/plant/unknown_plant")
        .to_request();

    let response = call_service(&app, request).await;

    common::assert_status_not_found(&response);
}
