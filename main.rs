use crate::api::encounter_api;
use crate::api::monster_api;
use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;

mod types {
    pub mod error;
    pub mod state;
    pub mod monster_params;
    pub mod encounter_params;
    pub mod monster;
}

mod external {
    pub mod encounter_json;
    pub mod monster_json;
}

mod internal {
    pub mod encounter;
}
mod api {
    pub mod encounter_api;
    pub mod monster_api;
}

mod handlers {
    pub mod query;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default()) // Optional: Use built-in request logger
            .service(encounter_api::get_encounter)
            .service(monster_api::get_monster)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
