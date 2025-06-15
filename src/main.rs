use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod database;
mod error;
mod models;
mod routes;

use database::Database;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::get_quotes,
        routes::get_quote_by_id,
        routes::create_quote,
        routes::update_quote,
        routes::delete_quote,
        routes::search_quotes,
    ),
    components(schemas(
        models::Quote,
        models::QuoteInput,
        models::QuoteWithTags,
        models::Tag,
        error::AppError,
    )),
    tags(
        (name = "quotes", description = "Quote management API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "quote_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // connect with database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://db/quotes.db".to_string());
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    let db = Database::new(pool);

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(routes::index))
        .route("/quotes", get(routes::get_quotes))
        .route("/quotes", post(routes::create_quote))
        .route("/quotes/search", get(routes::search_quotes))
        .route("/quotes/:id", get(routes::get_quote_by_id))
        .route("/quotes/:id", axum::routing::put(routes::update_quote))
        .route("/quotes/:id", axum::routing::delete(routes::delete_quote))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors)
        )
        .with_state(db);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}