use crate::result::Error;
use actix_web::web::{JsonConfig, QueryConfig};

pub struct Actix;

impl Actix {
    pub fn query_config() -> QueryConfig {
        QueryConfig::default().error_handler(|_, _| Error::MissingParams.into())
    }

    pub fn json_config() -> JsonConfig {
        JsonConfig::default().error_handler(|_, _| Error::MissingParams.into())
    }
}
