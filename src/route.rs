use crate::{api, controller};
use actix_web::web;

pub fn health(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health").service(controller::health::index));
}

pub fn api_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(api::v1::participle::cut)
            .service(api::v1::participle::cut_all),
    );
}
