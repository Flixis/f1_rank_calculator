// This module is only compiled when the `ssr` feature is enabled
#[cfg(feature = "ssr")]
pub mod get_requests {
    use actix_web::{get, HttpResponse, Responder, web};
    use sqlx::{MySqlPool, Error as SqlxError};
    use serde::Serialize;

    #[derive(Serialize, sqlx::FromRow)]
    struct Qualifying {
        // #[serde(rename = "qualifyId")]
        qualifyId: i32,
        // #[serde(rename = "raceId")]
        raceId: i32,
        // #[serde(rename = "driverId")]
        driverId: i32,
        // #[serde(rename = "constructorId")]
        constructorId: i32,
        number: i32,
        position: i32,
        q1: String,
        q2: String,
        q3: String,
    }
    

    #[get("/api/qualifying")]
    pub async fn get_qualifying(pool: web::Data<MySqlPool>) -> impl Responder {
        let result = sqlx::query_as::<_, Qualifying>("SELECT * FROM f1_database.qualifying LIMIT 5")
            .fetch_all(pool.get_ref())
            .await;
    
        match result {
            Ok(qualifying) => HttpResponse::Ok().json(qualifying),
            Err(e) => {
                eprintln!("Database error: {:?}", e); // Log the error
                HttpResponse::InternalServerError().finish()
            },
        }
    }
}