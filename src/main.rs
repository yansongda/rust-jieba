use crate::config::actix::Actix;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use jieba_rs::Jieba;

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
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(Actix::query_config())
            .data(Jieba::new())
            .configure(route::health)
            .configure(route::api_v1)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
