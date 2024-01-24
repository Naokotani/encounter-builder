use crate::encounter;
use crate::query;
use crate::encounter_api;
use crate::encounter::EitherBool;
use crate::error;
use crate::monster;
use sqlx::postgres::PgPoolOptions;
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


    // monster_types: &Vec<String>,
    // params: &encounter::MonsterGroup,
    // level: i32,
    // pool: &PgPool,

// pub struct MonsterGroup {
//     pub number: i32,
//     pub is_ranged: EitherBool,
//     pub is_caster: EitherBool,
// }

impl MonsterJson {
    async fn new(query_params: web::Query<QueryParams>) -> MonsterJson {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("DATABASE_URL").expect("Env var didn't load"))
            .await.unwrap();

        let is_ranged = query_params.is_ranged.as_str();
        let is_caster = query_params.is_ranged.as_str();

        let monster_group = encounter::MonsterGroup {
            number: 1,
            is_ranged: encounter_api::parse_either_bool(is_ranged).unwrap(),
            is_caster: encounter_api::parse_either_bool(is_caster).unwrap(),
        };

        let monster_types: Vec<String> = query_params.monster_types
            .split(',')
            .map(|v| v.to_string())
            .collect();
        println!("{:?}", monster_types);
        println!("{:?}", query_params.level);
        println!("{:?}", query_params);
        let monster = query::query(&monster_types, &monster_group, query_params.level, &pool).await;

        let monster = match monster {
            Ok(m) => m,
            Err(_) => panic!("Couldn't retrieve monsters {:?}", query_params),
        };

        if let Some(m) = monster {
            println!("{:?}", m);
            MonsterJson {
                budget: 10.0,
                url: m.url,
                name: m.name,
                number: m.number,
                level: m.level,
                alignment: m.alignment,
                monster_type: m.monster_type,
                aquatic: false,
                is_caster: m.is_caster,
                is_ranged: m.is_ranged,
                is_found: true,
            }
        } else {
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

