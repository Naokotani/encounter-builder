use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, middleware};
use dotenv::dotenv;
mod monster;
mod encounter;
mod encounter_api;
mod monster_api;
mod error;


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
