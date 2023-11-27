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
    

    #[get("/api/v1/f1/drivers/{driver_name}")]
    pub async fn get_driver_information(
        driver_name: web::Path<String>, 
        pool: web::Data<MySqlPool>,
    ) -> impl Responder {

        let query = format!("SELECT *
        FROM f1_database.drivers
        WHERE driverRef = \"{}\"
        LIMIT 25;", driver_name);
    
        let result = sqlx::query_as::<_, DriverInfo>(&query)
            .fetch_all(pool.get_ref())
            .await;

        match result {
            Ok(mut drivers) => {
                if let Some(driver) = drivers.pop() {
                    HttpResponse::Ok().json(driver)
                } else {
                    HttpResponse::NotFound().body("Driver not found")
                }
            }
            Err(e) => {
                eprintln!("Database error: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
    

}