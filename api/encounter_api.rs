use crate::external::encounter_json;
use actix_web::{
    get, web, HttpResponse,
};

#[get("/encounter")]
async fn get_encounter(query_params: web::Query<encounter_json::QueryParams>) -> HttpResponse {
    let encounter_json = encounter_json::EncounterJson::new(query_params).await;
    match encounter_json {
        Ok(json) => HttpResponse::Ok().json(json),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

