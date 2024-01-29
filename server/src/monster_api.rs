use crate::encounter;
use crate::encounter_api;
use crate::query;
use actix_web::{
    body::BoxBody, get, http::header::ContentType, web, HttpRequest, HttpResponse, Responder
};
use serde::Deserialize;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;

#[get("/monster")]
async fn get_monster(query_params: web::Query<QueryParams>) -> impl Responder {
    MonsterJson::new(query_params).await
}

#[derive(Deserialize, Debug)]
struct QueryParams {
    level: i32,
    party_level: i32,
    number: i32,
    monster_types: String,
    budget: i32,
    is_caster: String,
    is_ranged: String,
    bbeg: bool,
}

#[derive(Serialize)]
struct MonsterJson {
    budget: i32,
    url: String,
    name: String,
    number: i32,
    level: i32,
    alignment: String,
    monster_type: String,
    aquatic: bool,
    is_caster: bool,
    is_ranged: bool,
    status: String,
}

impl MonsterJson {
    async fn new(query_params: web::Query<QueryParams>) -> MonsterJson {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("DATABASE_URL").expect("Database connection falure."))
            .await
            .unwrap();

        let is_ranged = query_params.is_ranged.as_str();
        let is_caster = query_params.is_caster.as_str();
        let level_mod = query_params.level - query_params.party_level;
        let party_level = query_params.party_level;
        let budget = query_params.budget;

        let monster_group = encounter::MonsterGroup {
            number: query_params.number,
            is_ranged: encounter_api::parse_either_bool(is_ranged).unwrap(),
            is_caster: encounter_api::parse_either_bool(is_caster).unwrap(),
        };

        let monster_budget = if query_params.bbeg {
            println!("Bbeg: {}", query_params.bbeg);
            MonsterBudget {
                level: query_params.level,
                budget,
                number: 1,
            }
        } else if level_mod == -3 ||  level_mod == -4 {
          lackey_budget(party_level, level_mod, budget)
        } else if level_mod <= 0 || level_mod >= -2 {
          henchman_budget(party_level, level_mod, budget)
        } else {
            panic!("Unable to get budget for: {}", query_params.number)
        };

        println!("{}", query_params.number);
        println!("level: {}", query_params.level);
        println!("{}", query_params.budget);


        println!("{:?}", monster_budget);
        let monster_types: Vec<String> = query_params
            .monster_types
            .split(',')
            .map(|v| v.to_string())
            .collect();
        let monster = query::query(&monster_types, &monster_group,  monster_budget.level, &pool).await;

        let monster = match monster {
            Ok(m) => m,
            Err(_) => panic!("Couldn't retrieve monsters {:?}", query_params),
        };

        if let Some(m) = monster {
            println!("{:?}", m);
            MonsterJson {
                budget: monster_budget.budget,
                url: m.url,
                name: m.name,
                number: monster_budget.number,
                level: monster_budget.level,
                alignment: m.alignment,
                monster_type: m.monster_type,
                aquatic: false,
                is_caster: m.is_caster,
                is_ranged: m.is_ranged,
                status: String::from("Filled"),
            }
        } else {
            MonsterJson {
                budget: 10,
                url: String::from(""),
                name: String::from("Failed To find Monster"),
                number: 0,
                level: 0,
                alignment: String::from(""),
                monster_type: String::from(""),
                aquatic: false,
                is_caster: false,
                is_ranged: false,
                status: String::from("Failed"),
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

#[derive(Debug)]
struct MonsterBudget {
    level: i32,
    budget: i32,
    number: i32,
}

fn lackey_budget(party_level: i32, level: i32, budget: i32) -> MonsterBudget {
    let lackey_mod = match level {
        -3 if party_level > 3 && budget >= 20 => -4,
        -4 => -3,
        -3 => -3,
        _ => panic!("Level mod out of range {}", level),
    };
    let mut budget = budget as f32;


    match lackey_mod {
        -4 => {
            budget -= budget % 10.0;
            let number = (budget / 10.0).round() as i32;
            MonsterBudget {
                budget: number * 10,
                level: party_level - 4,
                number,
            }
        }
        -3 => {
            budget -= budget % 15.0;
            let number = (budget / 15.0).round() as i32;
            MonsterBudget {
                budget: number * 15,
                level: party_level - 3,
                number,
            }
        }
        _ => panic!("invalid random range"),
    }
}

fn henchman_budget(party_level: i32, level: i32, budget: i32) -> MonsterBudget {
    let hench_mod = match level {
        -2 if budget >= 30 => -1,
        -1 if budget >= 40 => 0,
         0 if party_level > 1 => -2,
        _ if budget >= 30 => -1,
        -2 => -2,
        _ => panic!("ya dun goofed"),
    };

    let mut budget = budget as f32;

    match hench_mod {
        -2 => {
            budget -= budget % 20.0;
            let number = (budget / 20.0).round() as i32;
            MonsterBudget {
                budget: number * 20,
                level: party_level - 2,
                number,
            }
        }
        -1 => {
            budget -= budget % 30.0;
            let number = (budget / 30.0).round() as i32;
            MonsterBudget {
                budget: number * 30,
                level: party_level - 1,
                number,
            }
        }
        0 => {
            budget -= budget % 40.0;
            let number = (budget / 40.0).round() as i32;
            MonsterBudget {
                budget: number * 40,
                level: party_level,
                number,
            }
        }
        _ => panic!("invalid random range"),
    }
}
