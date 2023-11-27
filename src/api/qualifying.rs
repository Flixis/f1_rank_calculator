#![allow(non_snake_case)] //TODO: Figure out a way to rename the fields inside the structs for database to match rust-lang style
// This module is only compiled when the `ssr` feature is enabled
#[cfg(feature = "ssr")]
pub mod get_requests {
    use actix_web::{get, HttpResponse, Responder, web};
    use sqlx::MySqlPool;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, sqlx::FromRow)]
    #[allow(non_snake_case)]
    struct Qualifying {
        qualifyId: Option<i32>,
        raceId: Option<i32>,
        driverId: Option<i32>,
        constructorId: Option<i32>,
        number: Option<i32>,
        position: Option<i32>,
        //Might return NULL therefore option
        q1: Option<String>,
        q2: Option<String>,
        q3: Option<String>,
    }

    #[derive(Deserialize)]
    struct QualifyingQueryParams {
        limit: i32
    }

    #[get("/api/v1/f1/qualifying")]
    pub async fn get_qualifying(
        pool: web::Data<MySqlPool>,
        query_params: web::Query<QualifyingQueryParams>,
    ) -> impl Responder {
        // Extract query parameters from the `query_params` object
        let limit = query_params.limit;
        // Add more parameters as needed
    
        let query = format!("SELECT * FROM f1_database.qualifying LIMIT {}", limit);
    
        let result = sqlx::query_as::<_, Qualifying>(&query)
            .fetch_all(pool.get_ref())
            .await;
    
        match result {
            Ok(qualifying) => HttpResponse::Ok().json(qualifying),
            Err(e) => {
                eprintln!("Database error: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }   

}