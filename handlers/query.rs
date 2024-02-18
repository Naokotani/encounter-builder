use crate::types::monster_params;
use tracing::{event, span, Level};
use crate::types::state::{EitherBool, FillStatus};
use crate::types::monster;
use sqlx::postgres::PgRow;
use sqlx::Execute;
use sqlx::PgPool;
use sqlx::Postgres;
use sqlx::QueryBuilder;
use sqlx::Row;

pub async fn query(
    monster_types: &Vec<String>,
    params: &monster_params::MonsterParams,
    upper: i32,
    lower: i32,
    pool: &PgPool,
    name: Option<&String>,
) -> Result<Option<Vec<monster::MonsterData>>, Box<dyn std::error::Error>> {

    if params.status == FillStatus::Skipped {
        return Ok(None);
    }

let span = span!(Level::TRACE, "database call");
let _enter = span.enter();

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
WHERE level >= ",
    );

    builder.push_bind(lower);
    builder.push("\nAND level <= ");
    builder.push_bind(upper);

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

    let monster_data = sqlx::query_with(sql, arguments)
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

    if monster_data.is_empty() {
        event!(Level::ERROR, status = "query fail between", upper, lower);
        return Ok(None);
    }

    Ok(Some(monster_data))
}
