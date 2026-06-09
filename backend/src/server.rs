use std::{
    collections::hash_map::HashMap,
    io::Error,
    sync::{Arc, RwLock},
};

use axum::Router;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::{config::load_config, domain::state::AppState, http::routes};

pub async fn serve() {
    let state = Arc::new(AppState {
        conf: load_config(),
        users: RwLock::new(HashMap::new()),
        stock: RwLock::new(HashMap::new()),
        provisional_stock: RwLock::new(HashMap::new()),
        baskets: RwLock::new(HashMap::new()),
    });

    init_server(state)
        .await
        .expect("Server initialization failed")
}

async fn init_server(state: Arc<AppState>) -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", state.conf.host, state.conf.port)).await?;

    let api_routes = Router::new()
        .merge(routes::calculator::calc_routes())
        .merge(routes::auth::auth_routes())
        .merge(routes::shop::shop_routes())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    axum::serve(listener, api_routes.into_make_service()).await?;
    Ok(())
}
