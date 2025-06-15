use crate::error::AppError;
use crate::models::{Quote, QuoteInput, Tag, QuoteWithTags};
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;

pub type DbPool = Pool<Sqlite>;

#[derive(Clone)]
pub struct Database {
    pool: DbPool,
}

impl Database {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_quotes(&self) -> Result<Vec<QuoteWithTags>, AppError> {
        let quotes = sqlx::query_as::<_, Quote>("SELECT id, text, author, source FROM quotes ORDER BY author, text")
            .fetch_all(&self.pool)
            .await?;

        let mut quote_map: HashMap<String, QuoteWithTags> = HashMap::new();
        for quote in quotes {
            quote_map.insert(quote.id.clone(), QuoteWithTags {
                quote,
                tags: Vec::new(),
            });
        }

        let tags = sqlx::query_as::<_, Tag>("SELECT quote_id, tag FROM tags ORDER BY quote_id, tag")
            .fetch_all(&self.pool)
            .await?;

        for tag in tags {
            if let Some(quote_with_tags) = quote_map.get_mut(&tag.quote_id) {
                quote_with_tags.tags.push(tag.tag);
            }
        }

        Ok(quote_map.into_values().collect())
    }

    pub async fn get_quote_by_id(&self, id: &str) -> Result<QuoteWithTags, AppError> {
        let quote = sqlx::query_as::<_, Quote>("SELECT id, text, author, source FROM quotes WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Quote with id {} not found", id)))?;

        let tags = sqlx::query_as::<_, Tag>("SELECT quote_id, tag FROM tags WHERE quote_id = ? ORDER BY tag")
            .bind(id)
            .fetch_all(&self.pool)
            .await?;

        let tag_strings: Vec<String> = tags.into_iter().map(|t| t.tag).collect();

        Ok(QuoteWithTags {
            quote,
            tags: tag_strings,
        })
    }

    pub async fn create_quote(&self, input: QuoteInput, tags: Vec<String>) -> Result<QuoteWithTags, AppError> {
        let quote = Quote::from(input);
        
        let mut tx = self.pool.begin().await?;

        sqlx::query("INSERT INTO quotes (id, text, author, source) VALUES (?, ?, ?, ?)")
            .bind(&quote.id)
            .bind(&quote.text)
            .bind(&quote.author)
            .bind(&quote.source)
            .execute(&mut *tx)
            .await?;

        for tag in &tags {
            sqlx::query("INSERT INTO tags (quote_id, tag) VALUES (?, ?)")
                .bind(&quote.id)
                .bind(tag)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;

        Ok(QuoteWithTags {
            quote,
            tags,
        })
    }

    pub async fn update_quote(&self, id: &str, input: QuoteInput, tags: Vec<String>) -> Result<QuoteWithTags, AppError> {
        let mut tx = self.pool.begin().await?;

        let result = sqlx::query("UPDATE quotes SET text = ?, author = ?, source = ? WHERE id = ?")
            .bind(&input.text)
            .bind(&input.author)
            .bind(&input.source)
            .bind(id)
            .execute(&mut *tx)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Quote with id {} not found", id)));
        }

        // Delete existing tags
        sqlx::query("DELETE FROM tags WHERE quote_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        // Insert new tags
        for tag in &tags {
            sqlx::query("INSERT INTO tags (quote_id, tag) VALUES (?, ?)")
                .bind(id)
                .bind(tag)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;

        let quote = Quote {
            id: id.to_string(),
            text: input.text,
            author: input.author,
            source: input.source,
        };

        Ok(QuoteWithTags {
            quote,
            tags,
        })
    }

    pub async fn delete_quote(&self, id: &str) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await?;

        // Delete tags first (foreign key constraint)
        sqlx::query("DELETE FROM tags WHERE quote_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        let result = sqlx::query("DELETE FROM quotes WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Quote with id {} not found", id)));
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn search_quotes(&self, author: Option<&str>, tag: Option<&str>, search: Option<&str>) -> Result<Vec<QuoteWithTags>, AppError> {
        let mut query = "SELECT DISTINCT q.id, q.text, q.author, q.source FROM quotes q".to_string();
        let mut joins = Vec::new();
        let mut conditions = Vec::new();
        let mut bind_values = Vec::new();

        if tag.is_some() {
            joins.push(" LEFT JOIN tags t ON q.id = t.quote_id");
        }

        if let Some(author_filter) = author {
            conditions.push("q.author LIKE ?");
            bind_values.push(format!("%{}%", author_filter));
        }

        if let Some(tag_filter) = tag {
            conditions.push("t.tag = ?");
            bind_values.push(tag_filter.to_string());
        }

        if let Some(search_term) = search {
            conditions.push("(q.text LIKE ? OR q.author LIKE ? OR q.source LIKE ?)");
            let search_pattern = format!("%{}%", search_term);
            bind_values.push(search_pattern.clone());
            bind_values.push(search_pattern.clone());
            bind_values.push(search_pattern);
        }

        for join in joins {
            query.push_str(&join);
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(" ORDER BY q.author, q.text");

        let mut sqlx_query = sqlx::query_as::<_, Quote>(&query);
        for value in bind_values {
            sqlx_query = sqlx_query.bind(value);
        }

        let quotes = sqlx_query.fetch_all(&self.pool).await?;

        let mut quote_map: HashMap<String, QuoteWithTags> = HashMap::new();
        for quote in quotes {
            quote_map.insert(quote.id.clone(), QuoteWithTags {
                quote,
                tags: Vec::new(),
            });
        }

        if !quote_map.is_empty() {
            let quote_ids: Vec<String> = quote_map.keys().cloned().collect();
            let placeholders = quote_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            let tags_query = format!("SELECT quote_id, tag FROM tags WHERE quote_id IN ({}) ORDER BY quote_id, tag", placeholders);
            
            let mut sqlx_tags_query = sqlx::query_as::<_, Tag>(&tags_query);
            for id in quote_ids {
                sqlx_tags_query = sqlx_tags_query.bind(id);
            }

            let tags = sqlx_tags_query.fetch_all(&self.pool).await?;

            for tag in tags {
                if let Some(quote_with_tags) = quote_map.get_mut(&tag.quote_id) {
                    quote_with_tags.tags.push(tag.tag);
                }
            }
        }

        Ok(quote_map.into_values().collect())
    }
}