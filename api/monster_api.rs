use crate::external::monster_json;
use serde::Deserialize;
use actix_web::{
    get, web, Responder
};


#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub name: String,
    pub level: i32,
    pub party_level: i32,
    pub number: i32,
    pub monster_types: String,
    pub budget: i32,
    pub is_caster: String,
    pub is_ranged: String,
    pub is_aquatic: bool,
    pub bbeg: bool,
}

#[get("/monster")]
async fn get_monster(query_params: web::Query<QueryParams>) -> impl Responder {

    let query_params = query_params.into_inner();
    monster_json::MonsterJson::new(query_params).await
}

