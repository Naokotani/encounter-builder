use crate::encounter;
use sqlx::postgres::PgPoolOptions;
use sqlx::QueryBuilder;
use sqlx::Postgres;
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
struct QueryParams {}

#[derive(Serialize)]
struct MonsterJson {}

impl MonsterJson {
    pub async fn new(query_params: web::Query<QueryParams>) -> Self{
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("DATABASE_URL").expect("Env var didn't load"))
            .await;

        let level = 5;

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

        builder.push_bind(level);

        // match params.is_caster {
        //     EitherBool::True => {
        //         builder.push("\nAND is_caster = TRUE");
        //     }

        //     EitherBool::False => {
        //         builder.push("\nAND is_caster = NOT TRUE");
        //     }
        //     _ => (),
        // };

        // match params.is_ranged {
        //     EitherBool::True => {
        //         builder.push("\nAND is_ranged = TRUE");
        //     }

        //     EitherBool::False => {
        //         builder.push("\nAND is_ranged = NOT TRUE");
        //     }
        //     _ => (),
        // };

        // let mut added_type = false;
        // for t in &self.monster_types {
        //     if !added_type {
        //         builder.push("\nAND (monster_type = ");
        //         builder.push_bind(t);
        //     } else {
        //         builder.push("\nOR monster_type = ");
        //         builder.push_bind(t);
        //     }
        //     added_type = true
        // }

        // if added_type {
        //     builder.push(")");
        // }

        // builder.push(";");

        // let mut query = builder.build();
        // let arguments = query.take_arguments().unwrap();
        // let sql = query.sql();

        // let mut monster_data = sqlx::query_with(sql, arguments)
        //     .map(|row: PgRow| MonsterData {
        //         creature_id: row.get(0),
        //         url: row.get(1),
        //         name: row.get(2),
        //         level: row.get(3),
        //         alignment: row.get(4),
        //         monster_type: row.get(5),
        //         size: row.get(6),
        //         is_caster: row.get(7),
        //         is_ranged: row.get(8),
        //         aquatic: row.get(9),
        //     })
        //     .fetch_all(pool)
        //     .await?;
        MonsterJson {} 
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

