use axum_app::app;
use axum_app::config::{
    logging::{layer::get_trace_layer, registry::init_logging},
    utils::handler_404,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    init_logging().await;

    let app = app().await
        .layer(get_trace_layer())
        .fallback(handler_404);

    let listner = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listner, app).await.unwrap();
}
