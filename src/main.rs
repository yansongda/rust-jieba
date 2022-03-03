use actix_web::{web, App, HttpServer};

use crate::config::actix::Actix;
use crate::config::config::Config;
use crate::config::jieba::JieBa;
use crate::config::logger::Logger;

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
    let config = Config::init();

    Logger::init(config.logger);

    HttpServer::new(|| {
        App::new()
            .app_data(Actix::query_config())
            .app_data(Actix::json_config())
            .app_data(web::Data::new(JieBa::init(config.jieba)))
            .configure(route::health)
            .configure(route::api_v1)
    })
    .bind(format!("{}:{}", config.app.listen, config.app.port))?
    .run()
    .await
}
