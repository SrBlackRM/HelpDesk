use std::sync::Arc;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, query, Row};

use super::appstate::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub category_id: Option<i32>,
    pub category_name: String,
}

impl Category {
    pub fn new_category(name: String) -> Self {
        Self {
            category_id: None,
            category_name: name,
        }
    }

    pub async fn save_new_category_in_db(new_category: &Self, state: Arc<AppState>) -> Result<(), sqlx::Error> {
        query(
            "INSERT INTO Categories (
                Category_Name
            ) VALUES (?)"
        )
        .bind(&new_category.category_name)
        .execute(&state.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all_categories(state: Arc<AppState>) -> Result<Vec<Category>, sqlx::Error> {
        let rows = query(
            "SELECT 
                ID_Category,
                Category_Name
            FROM Categories"
        )
        .fetch_all(&state.pool)
        .await?;

        let categories: Vec<Category> = rows.into_iter().map(|row| Category {
            category_id: row.try_get("ID_Category").unwrap_or(Some(0)),
            category_name: row.try_get("Category_Name").unwrap_or_else(|_| "".to_string()),
        }).collect();

        Ok(categories)
    }
}