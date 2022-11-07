mod models;
mod config;
mod handlers;
use actix_web::{HttpServer, App, web};
use crate::handlers::{get_person, status};
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let config = config::Config::from_env().unwrap();
    println!("Starting server at: {}:{}", config.server.host, config.server.port);

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    HttpServer::new(move ||{
        App::new()
            .app_data(pool.clone())
            .route("/", web::get().to(status))
            .route("/get_person", web::get().to(get_person))
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}