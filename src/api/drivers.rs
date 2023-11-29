#![allow(non_snake_case)]//TODO: Figure out a way to rename the fields inside the structs for database to match rust-lang style
// This module is only compiled when the `ssr` feature is enabled
#[cfg(feature = "ssr")]
pub mod get_requests {
    use actix_web::{get, HttpResponse, Responder, web};
    use sqlx::MySqlPool;
    use serde::{Serialize, Deserialize};


    #[derive(Serialize, Deserialize ,sqlx::FromRow)]
    #[allow(non_snake_case)]
    struct DriverInfo {
        driverId: Option<i32>,
        driverRef: Option<String>,
        number: Option<i32>,
        code: Option<String>,
        forename: Option<String>,
        surname: Option<String>,
        dob: Option<chrono::NaiveDate>,
        nationality: Option<String>,
        url: Option<String>,
    }
    

    #[get("/{driver_name}")]
    pub async fn get_driver(path: web::Path<String>) -> impl Responder {
        let driver_name = path.into_inner();
        HttpResponse::Ok().body(format!("Driver: {}", driver_name))
    }
    
    #[get("/{driver_name}/{constructor}")]
    pub async fn get_driver_and_constructor(path: web::Path<(String, String)>) -> impl Responder {
        let (driver_name, constructor) = path.into_inner();
        HttpResponse::Ok().body(format!("Driver: {}, Constructor: {}", driver_name, constructor))
    }
    
    #[get("/{driver_name}/{constructor}/{seasons}")]
    pub async fn get_driver_constructor_seasons(path: web::Path<(String, String, String)>) -> impl Responder {
        let (driver_name, constructor, seasons) = path.into_inner();
        HttpResponse::Ok().body(format!("Driver: {}, Constructor: {}, Seasons: {}", driver_name, constructor, seasons))
    }

}