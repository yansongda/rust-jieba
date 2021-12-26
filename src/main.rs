use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use jieba_rs::Jieba;

mod api;
mod controller;
mod route;
mod model;
mod config;
mod result;
mod response;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    HttpServer::new(|| {
        App::new().wrap(Logger::default())
            .data(Jieba::new())
            .configure(route::health)
            .configure(route::api_v1)
    }).bind("0.0.0.0:8080")?
        .run()
        .await
}
