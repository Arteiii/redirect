use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use crate::AppState;

pub(crate) async fn redirect(
    Path(link_type): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!("link: {:?}", link_type.as_str());

    let redirects = state.redirects.read().await;

    return if let Some(redirect_url) = redirects.get(&link_type) {
        let redirect_string = redirect_url.to_string();
        
        (
            StatusCode::PERMANENT_REDIRECT,
            [(header::LOCATION, redirect_string.clone())],
            redirect_string,
        )
    } else {
        let msg = format!("{} was not found", link_type).to_string();
        
        (
            StatusCode::NOT_FOUND,
            [(header::LOCATION, msg.clone())],
            msg,
        )
    }
}
