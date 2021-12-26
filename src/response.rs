use std::future::{ready, Ready};

use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{header, StatusCode};
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use serde::Serialize;

use crate::result::{Error, Response, ERROR_CODE_MESSAGE};

impl<D: Serialize> Responder for Response<D> {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&self).unwrap())))
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }

    fn error_response(&self) -> HttpResponse {
        let (code, message) = ERROR_CODE_MESSAGE
            .get(self)
            .unwrap_or_else(|| ERROR_CODE_MESSAGE.get(&Error::UnknownError).unwrap());
        let response: Response<String> = Response {
            code: *code,
            message: message.to_string(),
            data: None,
        };

        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .body(serde_json::to_string(&response).unwrap())
    }
}
