use crate::handlers::query;
use crate::internal::monster_budget::MonsterBudget;
use crate::monster_api;
use crate::types::state::{EitherBool, Weight};
use crate::types::{monster, monster_params};
use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use rand::Rng;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use tracing::{event, Level};

#[derive(Serialize, Debug)]
pub struct MonsterJson {
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
    pub async fn new(query_params: monster_api::QueryParams) -> MonsterJson {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("DATABASE_URL").expect("Database connection falure."))
            .await
            .unwrap();

        let is_ranged = query_params.is_ranged.as_str();
        let is_caster = query_params.is_caster.as_str();
        let party_level = query_params.party_level;
        let level = query_params.level;
        let budget = query_params.budget;
        let is_aquatic = query_params.is_aquatic;

        let monster_group = monster_params::MonsterParams::new(
            EitherBool::new(is_ranged).unwrap(),
            EitherBool::new(is_caster).unwrap(),
            is_aquatic,
            Weight::All,
        );

        let (upper, lower) = match level {
            l if query_params.bbeg => (l, l),
            l if l == party_level - 3 || l == party_level - 4 => (party_level - 3, party_level - 4),
            l if l <= party_level || l <= party_level - 2 => (party_level, party_level - 2),
            l => panic!("Invalid level mod {}", l),
        };

        let monster_types: Vec<String> = query_params
            .monster_types
            .split(',')
            .map(|v| v.to_string())
            .collect();
        let monster = query::query(&monster_types, &monster_group, upper, lower, &pool, None).await;

        let monster = match monster {
            Ok(m) => m,
            Err(_) => panic!("Couldn't retrieve monsters {:?}", query_params),
        };

        let mut monster = monster.unwrap();

        let mut rng = rand::thread_rng();

        let index = rng.gen_range(0..monster.len());
        let m =
            monster::Monster::new(monster.remove(index), vec![String::from("Undead")], 1).unwrap();

        let m = if m.name == query_params.name && !monster.is_empty() {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..monster.len());

            monster::Monster::new(monster.remove(index), vec![String::from("Undead")], 1).unwrap()
        } else {
            m
        };

        let monster_budget = match m.level {
            l if query_params.bbeg => MonsterBudget::bbeg_budget(l, budget),
            l if l == party_level - 3 || l == party_level - 4 => {
                MonsterBudget::lackey_budget(party_level, l, budget)
            }
            l if l <= party_level || l <= party_level - 2 => {
                MonsterBudget::hench_budget(party_level, l, budget)
            }
            l => panic!("Invalid level mod {}", l),
        };

        event!(
            Level::INFO,
            m.name,
            monster_budget.number,
            monster_budget.level,
            status = "Filled"
        );

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

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn new_bbeg() {
        let query = monster_api::QueryParams {
            name: String::from("Centipede Swarm"),
            level: 3,
            party_level: 1,
            number: 1,
            monster_types: String::from("Animal"),
            budget: 80,
            is_caster: String::from("either"),
            is_ranged: String::from("either"),
            is_aquatic: false,
            bbeg: true,
        };

        let result = MonsterJson::new(query).await;

        assert_eq!(result.number, 1);
        assert_eq!(result.monster_type, "Animal");
        assert_ne!(result.name, "Failed To find Monster");
        assert_ne!(result.status, "Skipped");
    }

    #[tokio::test]
    async fn new_hench() {
        let query = monster_api::QueryParams {
            name: String::from("Flea Swarm"),
            level: 5,
            party_level: 5,
            number: 2,
            monster_types: String::from("Animal"),
            budget: 80,
            is_caster: String::from("either"),
            is_ranged: String::from("either"),
            is_aquatic: false,
            bbeg: false,
        };

        let result = MonsterJson::new(query).await;

        assert_eq!(result.monster_type, "Animal");
        assert_ne!(result.name, "Failed To find Monster");
        assert_ne!(result.status, "Skipped");
    }

    #[tokio::test]
    async fn new_lackey() {
        let query = monster_api::QueryParams {
            name: String::from("Giant Solifugid"),
            level: 1,
            party_level: 5,
            number: 8,
            monster_types: String::from("Animal"),
            budget: 80,
            is_caster: String::from("either"),
            is_ranged: String::from("either"),
            is_aquatic: false,
            bbeg: false,
        };

        let result = MonsterJson::new(query).await;

        assert_eq!(result.monster_type, "Animal");
        assert_ne!(result.name, "Failed To find Monster");
        assert_ne!(result.status, "Skipped");
    }

    #[tokio::test]
    async fn replace_new_lackey() {
        let query = monster_api::QueryParams {
            name: String::from("Giant Viper"),
            level: 2,
            party_level: 5,
            number: 5,
            monster_types: String::from("Animal"),
            budget: 80,
            is_caster: String::from("either"),
            is_ranged: String::from("either"),
            is_aquatic: false,
            bbeg: false,
        };

        let result = MonsterJson::new(query).await;

        assert_eq!(result.monster_type, "Animal");
        assert_ne!(result.name, "Failed To find Monster");
        assert_ne!(result.status, "Skipped");
    }
}
