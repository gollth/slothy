mod router;

use crate::router::homepage;
use actix_web::web::ServiceConfig;

pub fn server(config: &mut ServiceConfig) {
    config.service(homepage);
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::StatusCode,
        test::{call_service, init_service, read_body, TestRequest},
        web::Bytes,
        App,
    };

    #[actix_web::test]
    async fn test_default_request_leads_to_homepage() {
        let app = init_service(App::new().configure(server)).await;
        let request = TestRequest::default().to_request();

        let response = call_service(&app, request).await;
        assert_eq!(StatusCode::OK, response.status());
        assert_eq!(Bytes::from_static(b"Slothy"), read_body(response).await);
    }
}
