use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{oneshot, RwLock};
use tracing_subscriber::{EnvFilter, fmt};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use serde::Deserialize;
use tokio::time::sleep;

mod routes;

#[derive(Debug, Default)]
struct AppState {
    redirects: RwLock<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct RedirectConfig {
    redirects: HashMap<String, String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    println!("Starting...");
    init_tracing();

    let state = Arc::new(AppState::default());
    let state_clone = state.clone();
    let state_clone_for_logging = state.clone();

    let (_shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        read_config_periodically(state_clone, shutdown_rx).await;
    });

    tokio::spawn(async move {
        log_redirects_if_new(state_clone_for_logging).await;
    });

    let port = "8000";

    let origins = [
        "http://192.168.178.58".parse().unwrap(),
        "http://localhost".parse().unwrap(),
    ];

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &port))
        .await
        .unwrap();


    tracing::info!("Server Running on: {}", port);

    axum::serve(listener, routes::configure_routes(origins, state))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to run Axum server");

    Ok(())
}


//noinspection GrazieInspection
#[inline(always)]
fn init_tracing() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}


async fn read_config_periodically(state: Arc<AppState>, mut shutdown_rx: oneshot::Receiver<()>) {
    match read_config().await {
        Ok(config) => {
            let mut redirects = state.redirects.write().await;
            *redirects = config.redirects;
            tracing::info!("Initial configuration loaded");
        },
        Err(e) => {
            tracing::error!("Failed to read initial config: {}", e);
        }
    }

    loop {
        tokio::select! {
            _ = &mut shutdown_rx => break,
            _ = sleep(tokio::time::Duration::from_secs(60)) => {
                match read_config().await {
                    Ok(config) => {
                        let mut redirects = state.redirects.write().await;
                        *redirects = config.redirects;
                        tracing::debug!("Configuration updated");
                    },
                    Err(e) => {
                        tracing::error!("Failed to read config: {}", e);
                    }
                }
            },
        }
    }
}


async fn read_config() -> Result<RedirectConfig, Box<dyn std::error::Error + Send + Sync>> {
    let config_data = tokio::fs::read_to_string("redirect.conf").await?;
    let config: RedirectConfig = toml::from_str(&config_data)?;
    Ok(config)
}


async fn log_redirects_if_new(state: Arc<AppState>) {
    let mut last_logged_redirects = HashMap::new();

    loop {
        let redirects = state.redirects.read().await;

        if *redirects != last_logged_redirects {
            for (local, remote) in redirects.iter() {
                tracing::info!("/{} -> {}", local, remote);
            }
            last_logged_redirects.clone_from(&redirects);
        }
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
}
