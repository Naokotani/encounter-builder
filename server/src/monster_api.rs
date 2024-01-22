use crate::encounter;
use crate::encounter::EitherBool;
use crate::error;
use crate::monster;
use actix_web::{
    body::BoxBody, get, http::header::ContentType, HttpRequest, HttpResponse, Responder,
    Result, web,
};
use serde::Serialize;
use serde::Deserialize;

#[get("/monster")]
async fn get_monster(query_params: web::Query<QueryParams>) -> impl Responder {
    MonsterJson::new(query_params).await
}

#[derive(Deserialize, Debug)]
struct QueryParams {
    level: i32,
    party_size: i32,
    monster_types: String,
    budget: String,
    is_caster: String,
    is_ranged: String,
}

#[derive(Serialize)]
struct MonsterJson {
    budget: f32,
    url: String,
    name: String,
    number: i32,
    level: i32,
    alignment: String,
    monster_type: String,
    aquatic: bool,
    is_caster: bool,
    is_ranged: bool,
    is_found: bool,
}

impl MonsterJson {
    async fn new(query_params: web::Query<QueryParams>) -> MonsterJson {
        MonsterJson {
            budget: 10.0,
            url: String::from("foo"),
            name: String::from("foo"),
            number: 0,
            level: 0,
            alignment: String::from("foo"),
            monster_type: String::from("foo"),
            aquatic: false,
            is_caster: false,
            is_ranged: false,
            is_found: false,
        }
    }
}

impl Responder for MonsterJson {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

