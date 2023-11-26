// This module is only compiled when the `ssr` feature is enabled
#[cfg(feature = "ssr")]
pub mod get_requests {
    use actix_web::{get, HttpResponse, Responder, web};
    use sqlx::MySqlPool;
    use serde::{Serialize, Deserialize};


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