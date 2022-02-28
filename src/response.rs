use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::{header, StatusCode};
use serde::Serialize;

use crate::result::{Error, ERROR_CODE_MESSAGE, Response};

impl<D: Serialize> Responder for Response<D> {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(header::ContentType::json())
            .body(body)
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

        HttpResponse::build(self.status_code())
            .insert_header((header::CONTENT_TYPE, "application/json; charset=utf-8"))
            .body(serde_json::to_string(&response).unwrap())
    }
}
