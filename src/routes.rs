use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use askama_axum::Template;
use serde::Deserialize;
use utoipa::IntoParams;

use crate::database::Database;
use crate::error::AppError;
use crate::models::{QuoteInput, QuoteWithTags};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    quotes: Vec<QuoteWithTags>,
}

#[derive(Deserialize, IntoParams)]
pub struct SearchParams {
    author: Option<String>,
    tag: Option<String>,
    search: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateQuoteRequest {
    #[serde(flatten)]
    pub quote: QuoteInput,
    pub tags: Option<Vec<String>>,
}

pub async fn index(State(db): State<Database>) -> Result<Html<String>, AppError> {
    let quotes = db.get_all_quotes().await?;
    let template = IndexTemplate { quotes };
    Ok(Html(template.render().map_err(|e| AppError::InternalError(e.to_string()))?))
}

/// Get all quotes
#[utoipa::path(
    get,
    path = "/quotes",
    responses(
        (status = 200, description = "List all quotes successfully", body = Vec<QuoteWithTags>),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "quotes"
)]
pub async fn get_quotes(State(db): State<Database>) -> Result<Json<Vec<QuoteWithTags>>, AppError> {
    let quotes = db.get_all_quotes().await?;
    Ok(Json(quotes))
}

/// Get a quote by ID
#[utoipa::path(
    get,
    path = "/quotes/{id}",
    responses(
        (status = 200, description = "Quote found successfully", body = QuoteWithTags),
        (status = 404, description = "Quote not found", body = AppError)
    ),
    params(
        ("id" = String, Path, description = "Quote database id")
    ),
    tag = "quotes"
)]
pub async fn get_quote_by_id(
    Path(id): Path<String>,
    State(db): State<Database>,
) -> Result<Json<QuoteWithTags>, AppError> {
    let quote = db.get_quote_by_id(&id).await?;
    Ok(Json(quote))
}

/// Create a new quote
#[utoipa::path(
    post,
    path = "/quotes",
    request_body = CreateQuoteRequest,
    responses(
        (status = 201, description = "Quote created successfully", body = QuoteWithTags),
        (status = 400, description = "Invalid input", body = AppError)
    ),
    tag = "quotes"
)]
pub async fn create_quote(
    State(db): State<Database>,
    Json(payload): Json<CreateQuoteRequest>,
) -> Result<impl IntoResponse, AppError> {
    let tags = payload.tags.unwrap_or_default();
    let quote = db.create_quote(payload.quote, tags).await?;
    Ok((StatusCode::CREATED, Json(quote)))
}

/// Update a quote
#[utoipa::path(
    put,
    path = "/quotes/{id}",
    request_body = CreateQuoteRequest,
    responses(
        (status = 200, description = "Quote updated successfully", body = QuoteWithTags),
        (status = 404, description = "Quote not found", body = AppError)
    ),
    params(
        ("id" = String, Path, description = "Quote database id")
    ),
    tag = "quotes"
)]
pub async fn update_quote(
    Path(id): Path<String>,
    State(db): State<Database>,
    Json(payload): Json<CreateQuoteRequest>,
) -> Result<Json<QuoteWithTags>, AppError> {
    let tags = payload.tags.unwrap_or_default();
    let quote = db.update_quote(&id, payload.quote, tags).await?;
    Ok(Json(quote))
}

/// Delete a quote
#[utoipa::path(
    delete,
    path = "/quotes/{id}",
    responses(
        (status = 204, description = "Quote deleted successfully"),
        (status = 404, description = "Quote not found", body = AppError)
    ),
    params(
        ("id" = String, Path, description = "Quote database id")
    ),
    tag = "quotes"
)]
pub async fn delete_quote(
    Path(id): Path<String>,
    State(db): State<Database>,
) -> Result<StatusCode, AppError> {
    db.delete_quote(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Search quotes
#[utoipa::path(
    get,
    path = "/quotes/search",
    params(SearchParams),
    responses(
        (status = 200, description = "Search completed successfully", body = Vec<QuoteWithTags>),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "quotes"
)]
pub async fn search_quotes(
    Query(params): Query<SearchParams>,
    State(db): State<Database>,
) -> Result<Json<Vec<QuoteWithTags>>, AppError> {
    let quotes = db.search_quotes(
        params.author.as_deref(),
        params.tag.as_deref(),
        params.search.as_deref(),
    ).await?;
    Ok(Json(quotes))
}