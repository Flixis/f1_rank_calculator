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
    

    #[get("/drivers/{driver_name}/{tail:.*}")]
    async fn get_driver_information(path: web::Path<(String, Option<String>)>) -> impl Responder {
        let (driver_name, tail) = path.into_inner();

        let constructor = if tail.clone().expect("something went wrong").is_empty() {
            None
        } else {
            Some(tail)
        };
    
        match constructor {
            Some(constructor) => HttpResponse::Ok().body(format!("Driver: {}, Constructor: {}", driver_name, constructor.expect("REASON").to_string())),
            None => HttpResponse::Ok().body(format!("Driver: {}", driver_name))
        }
    }

}