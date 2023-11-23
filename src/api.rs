// This module is only compiled when the `ssr` feature is enabled
#[cfg(feature = "ssr")]
pub mod get_requests {
    use actix_web::{get, HttpResponse, Responder};
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Response {
        pub message: String,
    }

    #[get("/health")]
    pub async fn healthcheck() -> impl Responder {
        let response = Response {
            message: "SSR is working fine".to_string(),
        };
        HttpResponse::Ok().json(response)
    }

    #[get("/api/health")]
    pub async fn api_health() -> impl Responder {
        let response = Response {
            message: "From api is working fine".to_string(),
        };
        HttpResponse::Ok().json(response)
    }
}