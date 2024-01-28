use crate::encounter;
use crate::error;
use crate::monster;
use actix_web::{
    body::BoxBody, get, http::header::ContentType, web, HttpRequest, HttpResponse, Responder,
    Result,
};
use serde::Deserialize;
use serde::Serialize;

#[get("/encounter")]
async fn get_encounter(query_params: web::Query<QueryParams>) -> impl Responder {
    EncounterJson::new(query_params).await
}

#[derive(Deserialize, Debug)]
struct QueryParams {
    level: i32,
    party_size: i32,
    difficulty: String,
    monster_types: String,
    bbeg_budget: String,
    bbeg_caster: String,
    bbeg_ranged: String,
    hench_budget: String,
    hench_caster: String,
    hench_ranged: String,
    lackey_budget: String,
    lackey_caster: String,
    lackey_ranged: String,
}

#[derive(Serialize)]
struct EncounterJson {
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
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

impl EncounterJson {
    async fn new(query_params: web::Query<QueryParams>) -> Self {
        let data = get_data(query_params).await;
        let (monsters, encounter) = data.unwrap();

        println!("{}", encounter);

        for monster in &monsters {
            println!("{}", monster);
        }

        let lackey_level = encounter.lackey_level.unwrap_or(encounter.level - 3);
        let (lackey, monsters) = get_monster(&encounter.lackey_status, lackey_level, monsters);

        let hench_level = encounter.hench_level.unwrap_or(encounter.level);
        let (henchmen, monsters) = get_monster(&encounter.hench_status, hench_level, monsters);

        let bbeg_level = encounter.bbeg_level.unwrap();
        let (bbeg, _) = get_monster(&encounter.bbeg_status, bbeg_level, monsters);

        let bbeg_status = parse_status(encounter.bbeg_status);
        let hench_status = parse_status(encounter.hench_status);
        let lackey_status = parse_status(encounter.lackey_status);

        EncounterJson {
            budget: encounter.budget,

            bbeg_budget: encounter.bbeg_budget,
            bbeg_url: bbeg.url,
            bbeg_name: bbeg.name,
            bbeg_number: bbeg.number,
            bbeg_level: bbeg.level,
            bbeg_alignment: bbeg.alignment,
            bbeg_monster_type: bbeg.monster_type,
            bbeg_aquatic: bbeg.aquatic,
            bbeg_is_caster: bbeg.is_caster,
            bbeg_is_ranged: bbeg.is_ranged,
            bbeg_status,

            hench_budget: encounter.hench_budget,
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

            lackey_budget: encounter.lackey_budget,
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
        }
    }

}

async fn get_data(
    query_params: web::Query<QueryParams>,
) -> Result<(Vec<monster::Monster>, encounter::Encounter), Box<dyn std::error::Error>> {
    let difficulty = match query_params.difficulty.as_str() {
        "trivial" => encounter::EncounterBudget::Trivial,
        "low" => encounter::EncounterBudget::Low,
        "moderate" => encounter::EncounterBudget::Moderate,
        "severe" => encounter::EncounterBudget::Severe,
        "extreme" => encounter::EncounterBudget::Extreme,
        _ => {
            return Err(Box::new(error::QueryError(String::from(
                "Bad difficulty string",
            ))))
        }
    };

    let monster_types: Vec<String> = query_params
        .monster_types
        .split(',')
        .map(|v| v.to_string())
        .collect();

    let bbeg_caster = parse_either_bool(&query_params.bbeg_caster).unwrap();
    let bbeg_ranged = parse_either_bool(&query_params.bbeg_ranged).unwrap();
    let hench_caster = parse_either_bool(&query_params.hench_caster).unwrap();
    let hench_ranged = parse_either_bool(&query_params.hench_ranged).unwrap();
    let lackey_caster = parse_either_bool(&query_params.lackey_caster).unwrap();
    let lackey_ranged = parse_either_bool(&query_params.lackey_ranged).unwrap();

    let configuration = encounter::Configuration {
        bbeg: parse_budget(&query_params.bbeg_budget).unwrap(),
        henchman: parse_budget(&query_params.hench_budget).unwrap(),
        lackey: parse_budget(&query_params.lackey_budget).unwrap(),
    };

    let mut encounter = encounter::Encounter {
        level: query_params.level,
        party_size: query_params.party_size,
        difficulty,
        monster_types,
        traits: Vec::new(),
        configuration,
        budget: 0.0,
        bbeg_budget: 0.0,
        bbeg_level: None,
        hench_budget: 0.0,
        hench_level: None,
        lackey_budget: 0.0,
        lackey_level: None,
        bbeg_status: encounter::FillStatus::Pending,
        bbeg_caster,
        bbeg_ranged,
        hench_status: encounter::FillStatus::Pending,
        hench_caster,
        hench_ranged,
        lackey_status: encounter::FillStatus::Pending,
        lackey_caster,
        lackey_ranged,
    };

    let monsters = encounter.create().await?;
    Ok((monsters, encounter))
}

pub fn parse_either_bool(
    bool_str: &str,
) -> Result<encounter::EitherBool, Box<dyn std::error::Error>> {
    let either_bool = match bool_str {
        "either" => encounter::EitherBool::Either,
        "true" => encounter::EitherBool::True,
        "false" => encounter::EitherBool::False,
        _ => {
            return Err(Box::new(error::QueryError(format!(
                "Bad bool string: {}",
                bool_str
            ))))
        }
    };
    Ok(either_bool)
}

pub fn parse_status(status: encounter::FillStatus) -> String {
    match status {
        encounter::FillStatus::Filled => String::from("Filled"),
        encounter::FillStatus::Pending => String::from("Pending"),
        encounter::FillStatus::Failed => String::from("Failed"),
        encounter::FillStatus::Skipped => String::from("Skipped"),
    }
}

fn parse_budget(budget: &str) -> Result<encounter::ConfigWeight, Box<dyn std::error::Error>> {
    let config_weight = match budget {
        "less" => encounter::ConfigWeight::Less,
        "even" => encounter::ConfigWeight::Even,
        "more" => encounter::ConfigWeight::More,
        "all" => encounter::ConfigWeight::All,
        "none" => encounter::ConfigWeight::None,
        _ => {
            return Err(Box::new(error::QueryError(format!(
                "Bad budget string: {}",
                budget
            ))))
        }
    };
    Ok(config_weight)
}

fn get_monster(status: &encounter::FillStatus, level: i32, mut monsters:Vec<monster::Monster>) -> (monster::Monster, Vec<monster::Monster>) {
    let monster = if status == &encounter::FillStatus::Filled {
        monsters.pop().unwrap()
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
    (monster, monsters)
}
