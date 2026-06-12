use std::io::Error;

use axum::Router;
use deadpool_diesel::{Manager, Runtime, postgres::Pool};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

use crate::{
    config::load_config,
    controllers::*,
    domain::state::AppState,
};

pub async fn serve() {
    let state = AppState {
        db: init_db(),
        conf: load_config(),
        // users: RwLock::new(HashMap::new()),
        // stock: RwLock::new(HashMap::new()),
        // provisional_stock: RwLock::new(HashMap::new()),
        // baskets: RwLock::new(HashMap::new()),
    };

    // migrate DB
    migrate_db(state.db.clone()).await;

    init_server(state)
        .await
        .expect("Server initialization failed")
}

async fn init_server(state: AppState) -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", state.conf.host, state.conf.port)).await?;

    let api_routes = Router::new()
        .merge(calculator::calc_routes())
        .merge(auth::auth_routes())
        .merge(shop::shop_routes())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    axum::serve(listener, api_routes.into_make_service()).await?;
    Ok(())
}

pub fn init_db() -> Pool {
    let db_url = "host=localhost user=postgres password=postgres dbname=backend-rs port=5432";
    let manager = Manager::new(db_url, Runtime::Tokio1);

    Pool::builder(manager).build().unwrap()
}

pub async fn migrate_db(db: Pool) {
    let con = db.get().await.unwrap();
    con.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap_or_else(|_| panic!("migration"));
}
