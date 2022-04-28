use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use tracing::Span;
use tracing_actix_web::RootSpanBuilder;

pub struct RequestLogger;

impl RootSpanBuilder for RequestLogger {
    fn on_request_start(request: &ServiceRequest) -> Span {
        let headers = request.headers();
        let request_id = tracing_actix_web::root_span_macro::private::get_request_id(request);
        let span = tracing_actix_web::root_span_macro::private::tracing::info_span!(
            "接收到请求",
            headers = ?headers,
            http.target = %request.uri().path_and_query().map(|p| p.as_str()).unwrap_or(""),
            trace_id = tracing_actix_web::root_span_macro::private::tracing::field::Empty,
            request_id = %request_id,
        );

        span
    }

    fn on_request_end<B>(_span: Span, _outcome: &Result<ServiceResponse<B>, Error>) {}
}
