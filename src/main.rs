mod config;
mod database;
mod handlers;
mod middleware;
mod models;
mod storage;

use axum::{
    middleware as axum_middleware,
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;
use database::create_pool;
use handlers::{
    delete_file, get_file_by_id_handler, list_files, serve_style_css, serve_upload_page,
    upload_file, AppState,
};
use middleware::create_auth_middleware;
use storage::Storage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "file_server_rs=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::parse_args();
    config.validate()?;

    tracing::info!("Starting file server with config: {:?}", config);

    let db_pool = create_pool(&config.database_url).await?;
    tracing::info!("Database connection established");

    let storage = Storage::from_config(&config).await?;
    tracing::info!("Storage initialized: {}", storage.storage_type());

    let app_state = AppState {
        db: db_pool,
        storage,
        config: Arc::new(config.clone()),
    };

    let mut app = Router::new()
        .route("/files/uploads/:id", get(get_file_by_id_handler))
        .route("/files/uploads", get(list_files))
        .route("/files/uploads/:id", delete(delete_file))
        .route("/style.css", get(serve_style_css))
        .with_state(app_state.clone());

    let upload_router = Router::new()
        .route("/upload", post(upload_file))
        .with_state(app_state.clone());

    if let Some(auth_token) = &config.auth_token {
        tracing::info!("Auth at /upload is enabled. This is recommended for prod.");
        app = app.merge(
            upload_router.layer(axum_middleware::from_fn(create_auth_middleware(
                auth_token.clone(),
            ))),
        );
    } else {
        tracing::warn!("Auth at /upload is disabled. This is not recommended for prod.");
        app = app.merge(upload_router);
    }

    if !config.disable_upload_page {
        tracing::warn!(
            "Running with upload page enabled, in prod you may wanna disable this. Set the env variable FILE_SERVER_DISABLE_UPLOAD_PAGE=true"
        );
        app = app.route("/", get(serve_upload_page));
    }

    app = app.layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::permissive()),
    );

    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("Server running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
