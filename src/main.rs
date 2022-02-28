use crate::config::actix::Actix;
use crate::config::jieba::JieBa;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

mod api;
mod config;
mod controller;
mod middleware;
mod model;
mod response;
mod result;
mod route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(Actix::query_config())
            .app_data(Actix::json_config())
            .app_data(web::Data::new(JieBa::init()))
            .configure(route::health)
            .configure(route::api_v1)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
