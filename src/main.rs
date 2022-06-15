use actix_web::{web, App, HttpServer};

use crate::config::actix::Actix;
use crate::config::jieba::JieBa;
use crate::config::logger::Logger;
use crate::config::CONFIG;

mod api;
mod config;
mod controller;
mod middleware;
mod model;
mod response;
mod result;
mod route;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_config(Logger::config()).unwrap();

    HttpServer::new(|| {
        App::new()
            .app_data(Actix::query_config())
            .app_data(Actix::json_config())
            .app_data(web::Data::new(JieBa::init()))
            .configure(route::health)
            .configure(route::api_v1)
    })
    .bind(format!("{}:{}", CONFIG.app.listen, CONFIG.app.port))?
    .run()
    .await
}
