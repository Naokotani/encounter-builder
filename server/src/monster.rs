use sqlx::postgres::PgPoolOptions;
use std::fmt;

#[derive(Debug)]
pub struct MonsterData {
    pub creature_id: Option<i32>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub level: Option<i32>,
    pub alignment: Option<String>,
    pub monster_type: Option<String>,
    pub size: Option<String>,
    pub aquatic: Option<bool>,
    pub is_caster: Option<bool>,
    pub is_ranged: Option<bool>,
}

pub struct Monster {
    pub creature_id: i32,
    pub url: String,
    pub name: String,
    pub number: i32,
    pub level: i32,
    pub alignment: String,
    pub monster_type: String,
    pub size: String,
    pub traits: Vec<String>,
    pub aquatic: bool,
    pub is_caster: bool,
    pub is_ranged: bool,
}

impl Monster {
    pub fn new(data: MonsterData, traits: Vec<String>, number: i32) -> Result<Monster, &'static str> {
        let creature_id = data.creature_id.ok_or("Failed to fetch creature id")?;
        let url = data.url.ok_or("Failed to fetch url")?;
        let name = data.name.ok_or("Failed to fetch name")?;
        let level = data.level.ok_or("Failed to fetch level")?;
        let alignment = data.alignment.ok_or("Failed to fetch alignment")?;
        let monster_type = data.monster_type.ok_or("Failed to fetch monster type")?;
        let size = data.size.ok_or("Failed to fetch size")?;
        let aquatic = data.aquatic.ok_or("Failed to fetch size")?;
        let is_caster = data.is_caster.ok_or("Failed to fetch is caster")?;
        let is_ranged = data.is_ranged.ok_or("Failed to fetch is ranged")?;

        Ok(Monster {
            creature_id,
            url,
            name,
            number,
            level,
            alignment,
            monster_type,
            size,
            traits,
            aquatic,
            is_caster,
            is_ranged,
        })
    }

    #[allow(dead_code)]
    pub async fn fetch_one(id: i32) -> Result<Monster, Box<dyn std::error::Error>> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("DATABASE_URL").expect("Env var didn't load"))
            .await?;

        let monster_data = sqlx::query_as!(
            MonsterData,
            "
SELECT
 m.creature_id,
 m.url,
 m.name,
 m.level,
 m.alignment,
 m.monster_type,
 m.size,
 m.aquatic,
 m.is_caster,
 m.is_ranged
FROM monsters_new m
WHERE m.creature_id = $1;
        ",
            id
        )
        .fetch_one(&pool)
        .await?;

        let trait_records = sqlx::query!(
            "
SELECT trait
FROM traits_new
WHERE creature_id = $1;
        ",
            monster_data.creature_id.expect("foo")
        )
        .fetch_all(&pool)
        .await?;

        let mut monster_traits: Vec<String> = Vec::new();

        for record in trait_records {
            if let Some(s) = record.r#trait {
                monster_traits.push(s)
            }
        }

        let monster = Monster::new(monster_data, monster_traits, 1).unwrap();

        Ok(monster)
    }
}

impl fmt::Display for Monster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
creatre id: {}
name: {}
url: {}
number: {}
level: {}
type: {}
alignment: {}
size: {}
traits: {:?}
aquatic: {}
Caster? {}
Ranged? {}\n",
            self.creature_id,
            self.name,
            self.url,
            self.number,
            self.level,
            self.monster_type,
            self.alignment,
            self.size,
            self.traits,
            self.aquatic,
            self.is_caster,
            self.is_ranged
        )
    }
}
