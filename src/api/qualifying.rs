// This module is only compiled when the `ssr` feature is enabled
#[cfg(feature = "ssr")]
pub mod get_requests {
    use actix_web::{get, HttpResponse, Responder, web};
    use sqlx::MySqlPool;
    use serde::{Serialize, Deserialize};

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
        //Might return NULL therefore option
        q1: Option<String>,
        q2: Option<String>,
        q3: Option<String>,
    }

    #[derive(Deserialize)]
    struct QualifyingQueryParams {
        limit: i32
    }

    #[get("/api/qualifying")]
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


    #[get("/api/v1/f1/{year}/circuits/{circuitId}")]
    pub async fn get_circuit_info(
        year: web::Path<(i32, String)>, 
        info: web::Path<(i32, String)>,
    ) -> impl Responder {
        // Extract `year` and `circuitId` from the URL path
        let year = info.0;
        let circuit_id = &info.1;
    
        // Your logic to fetch circuit information goes here
    
        // Return a response
        HttpResponse::Ok().body(format!("Circuit info for year {} and ID: {}", year, circuit_id))
    }
    

}