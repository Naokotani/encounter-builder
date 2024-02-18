use tracing::{event, Level};
use crate::api::encounter_api;
use crate::internal::encounter;
use crate::types::encounter_params;
use crate::types::error;
use crate::types::monster;
use crate::types::monster_params;
use crate::types::state::{EitherBool, EncounterBudget, FillStatus, Weight};
use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct EncounterJson {
    id: String,
    budget: f32,

    bbeg_budget: f32,
    bbeg_url: String,
    bbeg_name: String,
    bbeg_number: i32,
    bbeg_level: i32,
    bbeg_alignment: String,
    bbeg_monster_type: String,
    bbeg_aquatic: bool,
    bbeg_is_caster: bool,
    bbeg_is_ranged: bool,
    bbeg_status: String,

    hench_budget: f32,
    hench_url: String,
    hench_name: String,
    hench_number: i32,
    hench_level: i32,
    hench_alignment: String,
    hench_monster_type: String,
    hench_aquatic: bool,
    hench_is_caster: bool,
    hench_is_ranged: bool,
    hench_status: String,

    lackey_budget: f32,
    lackey_url: String,
    lackey_name: String,
    lackey_number: i32,
    lackey_level: i32,
    lackey_alignment: String,
    lackey_monster_type: String,
    lackey_aquatic: bool,
    lackey_is_caster: bool,
    lackey_is_ranged: bool,
    lackey_status: String,
}

impl Responder for EncounterJson {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        match serde_json::to_string(&self) {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(e) => {
                let error = format!("Failed to serialize json {}", e);
                HttpResponse::NotFound()
                    .content_type(ContentType::json())
                    .body(error)
            }
        }
    }
}

impl EncounterJson {
    pub async fn new(
        query_params: encounter_api::QueryParams,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let data = get_data(query_params).await;
        let (monsters, encounter) = data?;

        let lackey_level = encounter.lackey.level.unwrap_or(encounter.level - 3);
        let (lackey, monsters) = get_monster(&encounter.lackey.status, lackey_level, monsters)?;

        let hench_level = encounter.hench.level.unwrap_or(encounter.level);
        let (henchmen, monsters) = get_monster(&encounter.hench.status, hench_level, monsters)?;

        let (bbeg, _) = get_monster(
            &encounter.bbeg.status,
            encounter.bbeg.level.unwrap_or(0),
            monsters,
        )?;

        let bbeg_status = encounter.bbeg.status.stringify();
        let hench_status = encounter.hench.status.stringify();
        let lackey_status = encounter.lackey.status.stringify();

        let id = Uuid::new_v4();
        event!(Level::INFO, "Encounter created with id: {}", id);

        Ok(EncounterJson {
            id: id.to_string(),
            budget: encounter.budget,

            bbeg_budget: encounter.bbeg.budget,
            bbeg_url: bbeg.url,
            bbeg_name: bbeg.name,
            bbeg_number: bbeg.number,
            bbeg_level: encounter.bbeg.level.unwrap_or(0),
            bbeg_alignment: bbeg.alignment,
            bbeg_monster_type: bbeg.monster_type,
            bbeg_aquatic: bbeg.aquatic,
            bbeg_is_caster: bbeg.is_caster,
            bbeg_is_ranged: bbeg.is_ranged,
            bbeg_status,

            hench_budget: encounter.hench.budget,
            hench_url: henchmen.url,
            hench_name: henchmen.name,
            hench_number: henchmen.number,
            hench_level: henchmen.level,
            hench_alignment: henchmen.alignment,
            hench_monster_type: henchmen.monster_type,
            hench_aquatic: henchmen.aquatic,
            hench_is_caster: henchmen.is_caster,
            hench_is_ranged: henchmen.is_ranged,
            hench_status,

            lackey_budget: encounter.lackey.budget,
            lackey_url: lackey.url,
            lackey_name: lackey.name,
            lackey_number: lackey.number,
            lackey_level: lackey.level,
            lackey_alignment: lackey.alignment,
            lackey_monster_type: lackey.monster_type,
            lackey_aquatic: lackey.aquatic,
            lackey_is_caster: lackey.is_caster,
            lackey_is_ranged: lackey.is_ranged,
            lackey_status,
        })
    }
}

async fn get_data(
    query_params: encounter_api::QueryParams,
) -> Result<(Vec<monster::Monster>, encounter::Encounter), Box<dyn std::error::Error>> {
    let difficulty = EncounterBudget::new(query_params.difficulty.as_str())?;

    let monster_types: Vec<String> = query_params
        .monster_types
        .split(',')
        .map(|v| v.to_string())
        .collect();

    let bbeg_caster = EitherBool::new(&query_params.bbeg_caster)?;
    let bbeg_ranged = EitherBool::new(&query_params.bbeg_ranged)?;
    let hench_caster = EitherBool::new(&query_params.hench_caster)?;
    let hench_ranged = EitherBool::new(&query_params.hench_ranged)?;
    let lackey_caster = EitherBool::new(&query_params.lackey_caster)?;
    let lackey_ranged = EitherBool::new(&query_params.lackey_ranged)?;

    let monster_weights = encounter::MonsterWeights {
        bbeg: Weight::new(&query_params.bbeg_budget)?,
        henchman: Weight::new(&query_params.hench_budget)?,
        lackey: Weight::new(&query_params.lackey_budget)?,
    };

    let encounter_params = encounter_params::EncounterParams::new(
        query_params.level,
        query_params.party_size,
        difficulty,
        monster_types,
    );

    let bbeg_params =
        monster_params::MonsterParams::new( bbeg_caster, bbeg_ranged, query_params.bbeg_aquatic, monster_weights.bbeg);

    let hench_params =
        monster_params::MonsterParams::new(hench_caster, hench_ranged, query_params.hench_aquatic, monster_weights.henchman);

    let lackey_params = monster_params::MonsterParams::new(
        lackey_caster,
        lackey_ranged,
        query_params.lackey_aquatic,
        monster_weights.lackey
    );

    let mut encounter =
        encounter::Encounter::new(encounter_params, bbeg_params, hench_params, lackey_params);

    let monsters = encounter.create().await?;
    Ok((monsters, encounter))
}

fn get_monster(
    status: &FillStatus,
    level: i32,
    mut monsters: Vec<monster::Monster>,
) -> Result<(monster::Monster, Vec<monster::Monster>), Box<dyn std::error::Error>> {
    let monster = if status == &FillStatus::Filled {
        match monsters.pop() {
            Some(m) => m,
            None => return Err(Box::new(error::VecError::EmptyMonster(level))),
        }
    } else {
        monster::Monster {
            creature_id: 0,
            url: String::from("None"),
            name: String::from("Failed To find Monster"),
            number: 0,
            level,
            alignment: String::from("None"),
            monster_type: String::from("None"),
            size: String::from("None"),
            traits: vec![String::from("None")],
            aquatic: false,
            is_caster: false,
            is_ranged: false,
        }
    };
    Ok((monster, monsters))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn bbeg_solo() {
        let difficulties: [&str; 5] = ["trivial", "low", "moderate", "severe", "extreme"];
        for diff in difficulties.into_iter() {
            bbeg_solo_levels(diff).await;
        }
    }

    async fn bbeg_solo_levels(diff: &str) {
        event!(Level::INFO, diff);
        for i in 1..20 {
            event!(Level::INFO, group="Bbeg", i, diff);
            let query = encounter_api::QueryParams {
                level: i,
                party_size: 4,
                difficulty: String::from(diff),
                monster_types: String::from("Animal"),
                bbeg_budget: String::from("all"),
                bbeg_caster: String::from("either"),
                bbeg_ranged: String::from("either"),
                bbeg_aquatic: false,
                hench_budget: String::from("none"),
                hench_caster: String::from("either"),
                hench_ranged: String::from("either"),
                hench_aquatic: false,
                lackey_budget: String::from("none"),
                lackey_caster: String::from("either"),
                lackey_ranged: String::from("either"),
                lackey_aquatic: false,
            };

            let res = EncounterJson::new(query)
                .await
                .expect("no result from encounter json new");
            event!(Level::INFO, res.budget);

            assert!(res.bbeg_number > 0);
            assert!(matches!(res.bbeg_level, l if l <= i + 4 && l >= i - 4));
            assert_eq!(res.bbeg_status, "Filled");
        }
    }

    #[tokio::test]
    async fn hench_solo() {
        let difficulties: [&str; 5] = ["trivial", "low", "moderate", "severe", "extreme"];
        for diff in difficulties.into_iter() {
            hench_solo_levels(diff).await;
        }
    }

    async fn hench_solo_levels(diff: &str) {
        event!(Level::INFO, diff);
        for i in 1..20 {
            event!(Level::INFO, group="Henchman", i, diff);
            let query = encounter_api::QueryParams {
                level: i,
                party_size: 4,
                difficulty: String::from(diff),
                monster_types: String::from("Animal"),
                bbeg_budget: String::from("none"),
                bbeg_caster: String::from("either"),
                bbeg_ranged: String::from("either"),
                bbeg_aquatic: false,
                hench_budget: String::from("all"),
                hench_caster: String::from("either"),
                hench_ranged: String::from("either"),
                hench_aquatic: false,
                lackey_budget: String::from("none"),
                lackey_caster: String::from("either"),
                lackey_ranged: String::from("either"),
                lackey_aquatic: false,
            };

            let res = EncounterJson::new(query)
                .await
                .expect("no result from encounter json new");

            assert!(matches!(res.hench_level, l if l <= i && l >= i - 2));
            assert!((0.0..=20.0).contains(&res.budget));
            assert!(res.hench_number > 0);
            assert_eq!(res.hench_status, "Filled");
        }
    }

    #[tokio::test]
    async fn lackey_solo() {
        let difficulties: [&str; 5] = ["trivial", "low", "moderate", "severe", "extreme"];
        for diff in difficulties.into_iter() {
            lackey_solo_levels(diff).await;
        }
    }

    async fn lackey_solo_levels(diff: &str) {
        event!(Level::INFO, diff);
        for i in 2..20 {
            event!(Level::INFO, group="Henchman", i, diff);
            let query = encounter_api::QueryParams {
                level: i,
                party_size: 4,
                difficulty: String::from(diff),
                monster_types: String::from("Animal"),
                bbeg_budget: String::from("none"),
                bbeg_caster: String::from("either"),
                bbeg_ranged: String::from("either"),
                bbeg_aquatic: false,
                hench_budget: String::from("none"),
                hench_caster: String::from("either"),
                hench_ranged: String::from("either"),
                hench_aquatic: false,
                lackey_budget: String::from("all"),
                lackey_caster: String::from("either"),
                lackey_ranged: String::from("either"),
                lackey_aquatic: false,
            };

            let res = EncounterJson::new(query)
                .await
                .expect("no result from encounter json new");

            assert!(res.lackey_number > 0);
            assert!(matches!(res.lackey_level, l if l == i -3 || l == i - 4));
            assert!((0.0..=10.0).contains(&res.budget));
            assert_eq!(res.lackey_status, "Filled");
        }
    }
}
