use std::fmt;
use tracing::{event, Level};

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

#[derive(Debug)]
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
    pub fn new(
        data: MonsterData,
        traits: Vec<String>,
        number: i32,
    ) -> Result<Monster, &'static str> {
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

        event!(Level::INFO, status="Creating monster", name, number);
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

    pub fn empty() -> Self {
        Monster {
            creature_id: 0,
            url: String::from(""),
            name: String::from("Failed to find monster"),
            number: 0,
            level: 0,
            alignment: String::from(""),
            monster_type: String::from(""),
            size: String::from(""),
            traits: vec![String::from("")],
            aquatic: false,
            is_caster: false,
            is_ranged: false,
        }
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
