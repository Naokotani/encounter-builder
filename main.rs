use actix_web::{post, App, HttpResponse, HttpServer, Responder, middleware};
use dotenv::dotenv;
use crate::api::monster_api;
use crate::api::encounter_api;

mod types {
    pub mod monster;
    pub mod encounter;
    pub mod error;
}
mod api {
    pub mod encounter_api;
    pub mod monster_api;
    
}

mod handlers {
    pub mod query;
}


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
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
            .service(echo)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
