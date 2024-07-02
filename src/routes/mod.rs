use std::sync::Arc;
use std::time::Duration;

use axum::http::header::AUTHORIZATION;
use axum::http::Method;
use axum::{routing::get, Router};
use tower_http::cors::AllowOrigin;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};
use crate::AppState;

mod link;

pub fn configure_routes<T>(origins: T, state: Arc<AppState>) -> Router
where
    T: Into<AllowOrigin>,
{
    Router::new()
        .route("/:link_type", get(link::redirect))
        .with_state(state)
        .layer(TimeoutLayer::new(Duration::from_secs(90))) // abort request after 90sec
        .layer(
            CorsLayer::new()
                .allow_origin(origins)
                .allow_headers([AUTHORIZATION])
                .allow_methods([Method::GET, Method::POST, Method::PUT]),
        )
        .layer(TraceLayer::new_for_http())
}
