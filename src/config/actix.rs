use crate::result::Error;
use actix_web::web;
use actix_web::web::QueryConfig;

pub struct Actix;

impl Actix {
    pub fn query_config() -> QueryConfig {
        web::QueryConfig::default().error_handler(|_, _| Error::MissingParams.into())
    }
}
