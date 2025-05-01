use axum::{
    extract::State,
    response::Html,
    routing::get,
    Router,
};
use askama::Template;

use crate::models::Quote;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    quote: Quote,
}

pub fn app() -> Router<Quote> {
    Router::new()
        .route("/", get(pub_root_handler))
}

pub async fn pub_root_handler(State(quote): State<Quote>) -> Html<String> {
    let template = IndexTemplate { quote };
    Html(template.render().unwrap())
}