use actix_web::web;
use crate::{api, controller};

pub fn health(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health")
        .service(controller::health::index)
    );
}

pub fn api_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1")
        .service(api::v1::participle::index)
    );
}
