// Tera imports
use tera::Tera;

// SQLX imports
use sqlx::mysql::MySqlPool;


pub struct AppState{
    pub tera: Tera,
    pub pool: MySqlPool,
}