use crate::external::encounter_json;
use serde::Deserialize;
use actix_web::{
    get, web, HttpResponse,
};

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub level: i32,
    pub party_size: i32,
    pub difficulty: String,
    pub monster_types: String,
    pub bbeg_budget: String,
    pub bbeg_caster: String,
    pub bbeg_ranged: String,
    pub bbeg_aquatic: bool,
    pub hench_budget: String,
    pub hench_caster: String,
    pub hench_ranged: String,
    pub hench_aquatic: bool,
    pub lackey_budget: String,
    pub lackey_caster: String,
    pub lackey_ranged: String,
    pub lackey_aquatic: bool,
}

#[get("/encounter")]
async fn get_encounter(query_params: web::Query<QueryParams>) -> HttpResponse {
    let encounter_json = encounter_json::EncounterJson::new(query_params.into_inner()).await;
    match encounter_json {
        Ok(json) => HttpResponse::Ok().json(json),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

