use axum::{body::Body, http::{Request, Response}};
use sea_orm::prelude::Uuid;
use std::time::Duration;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::TraceLayer,
};
use tracing::{Span, info, info_span};

pub fn get_trace_layer() -> TraceLayer<
    SharedClassifier<ServerErrorsAsFailures>,
    impl Fn(&Request<Body>) -> Span + Clone + Send + Sync + 'static,
    impl Fn(&Request<Body>, &Span) + Clone + Send + Sync + 'static,
    impl Fn(&Response<Body>, std::time::Duration, &Span) + Clone + Send + Sync + 'static,
> {
    TraceLayer::new_for_http()
    .make_span_with(|_request: &Request<Body>| {
            let trace_id = Uuid::new_v4().as_simple().to_string();
            info_span!("http", %trace_id)
        })
        .on_request(|request: &Request<Body>, _span: &Span| {
            info!("Request - {} {}", request.method(), request.uri());
        })
        .on_response(|response: &Response<Body>, _latency: Duration, _span: &Span| {
            info!("Response - {}", response.status());
        })
}
