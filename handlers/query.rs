use crate::types::encounter;
use crate::types::encounter::EitherBool;
use crate::types::monster;
use rand::Rng;
use sqlx::postgres::PgRow;
use sqlx::Execute;
use sqlx::PgPool;
use sqlx::Postgres;
use sqlx::QueryBuilder;
use sqlx::Row;

pub async fn query(
    monster_types: &Vec<String>,
    params: &encounter::MonsterGroup,
    level: i32,
    pool: &PgPool,
    name: Option<&String>,
) -> Result<Option<monster::Monster>, Box<dyn std::error::Error>> {
    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "
SELECT
creature_id,
url,
name,
level,
alignment,
monster_type,
size,
aquatic,
is_caster,
is_ranged
FROM monsters_new
WHERE level = ",
    );

    println!("{}", params.is_aquatic);
    builder.push_bind(level);

    if params.is_aquatic {
        builder.push("\nAND aquatic = TRUE");
    }

    if let Some(name) = name {
        builder.push("\nAND name != ");
        builder.push_bind(name);
    }

    match params.is_caster {
        EitherBool::True => {
            builder.push("\nAND is_caster = TRUE");
        }

        EitherBool::False => {
            builder.push("\nAND is_caster = NOT TRUE");
        }
        _ => (),
    };

    match params.is_ranged {
        EitherBool::True => {
            builder.push("\nAND is_ranged = TRUE");
        }

        EitherBool::False => {
            builder.push("\nAND is_ranged = NOT TRUE");
        }
        _ => (),
    };

    let mut added_type = false;
    for t in monster_types {
        if !added_type {
            builder.push("\nAND (monster_type = ");
            builder.push_bind(t);
        } else {
            builder.push("\nOR monster_type = ");
            builder.push_bind(t);
        }
        added_type = true
    }

    if added_type {
        builder.push(")");
    }

    builder.push(";");

    let mut query = builder.build();
    let arguments = query.take_arguments().unwrap();
    let sql = query.sql();

    let mut monster_data = sqlx::query_with(sql, arguments)
        .map(|row: PgRow| monster::MonsterData {
            creature_id: row.get(0),
            url: row.get(1),
            name: row.get(2),
            level: row.get(3),
            alignment: row.get(4),
            monster_type: row.get(5),
            size: row.get(6),
            aquatic: row.get(7),
            is_caster: row.get(8),
            is_ranged: row.get(9),
        })
        .fetch_all(pool)
        .await?;

    let monster_traits = vec![String::from("Undead")];

    if monster_data.is_empty() {
        println!(
            "Search for level: {} caster: {:?} ranged: {:?} type: {:?} produced no results",
            level, params.is_caster, params.is_ranged, monster_types
        );
        return Ok(None);
    }

    let mut rng = rand::thread_rng();
    let random_monster = monster_data.remove(rng.gen_range(0..monster_data.len()));

    let monster = monster::Monster::new(random_monster, monster_traits, params.number).unwrap();
    Ok(Some(monster))
}