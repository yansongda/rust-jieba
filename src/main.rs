use actix_web::{App, HttpServer, web};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::actix::Actix;
use crate::config::CONFIG;
use crate::config::jieba::JieBa;
use crate::config::logger::Logger;
use crate::middleware::request_logger_middleware::RequestLogger;

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
    tracing_subscriber::registry().with(Logger).init();

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::<RequestLogger>::new())
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
