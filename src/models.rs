use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Quote {
    pub id: String,
    pub text: String,
    pub author: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct QuoteInput {
    pub text: String,
    pub author: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Tag {
    pub quote_id: String,
    pub tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct QuoteWithTags {
    #[serde(flatten)]
    pub quote: Quote,
    pub tags: Vec<String>,
}

impl Quote {
    pub fn new(text: String, author: String, source: String) -> Self {
        Self {
            id: Self::generate_id(),
            text,
            author,
            source,
        }
    }

    fn generate_id() -> String {
        format!("{:08x}", fastrand::u32(..))
    }
}

impl From<QuoteInput> for Quote {
    fn from(input: QuoteInput) -> Self {
        Quote::new(input.text, input.author, input.source)
    }
}