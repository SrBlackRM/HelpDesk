use serde::{Deserialize, Serialize};
// use sqlx::Row;
use sqlx::Type;
use sqlx::query;
// use crate::db::connection::establish_connection;
use crate::models::appstate::AppState;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: Option<i32>,
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
    pub user_phone: Option<String>,
    pub user_section: Option<String>,
    pub user_role: UserRole,
    pub user_expertise: Option<String>,
    pub user_active: bool
}

#[derive(Debug, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "ENUM('cliente', 'tecnico', 'administrador')", rename_all = "lowercase")]
pub enum UserRole {
    Cliente,
    Tecnico,
    Administrador,
}

impl User {
    pub fn build_user(name: String, email: String, password: String) -> Self {
        Self {
            user_id: None,
            user_name: name,
            user_email: email,
            user_password: password,
            user_phone: None,
            user_section: None,
            user_role: UserRole::Cliente, // Padrão como 'Cliente'
            user_expertise: None,
            user_active: false
        }
    }

    pub async fn save_user_in_db(new_user: &Self, state: Arc<AppState>){
        // Inserir no banco com a query corrigida
        query(
            "INSERT INTO Users (User_Name, User_Email, User_Password, User_Role) VALUES (?, ?, ?, ?)"
        )
        .bind(&new_user.user_name)
        .bind(&new_user.user_email)
        .bind(&new_user.user_password)
        .bind(format!("{:?}", new_user.user_role)) // Converte enum para string
        .execute(&state.pool)
        .await
        .expect("Erro ao inserir usuário");
    }


    // NOT USING THIS YET
    // pub async fn get_id_by_email(email: String) ->  Option<i32>{
    //     let sql = "SELECT ID_User FROM Users WHERE User_Email = ?";

    //     let pool = establish_connection().await.ok()?;
        
    //     match query(sql)
    //     .bind(email)
    //     .fetch_one(&pool)
    //     .await {
    //         Ok(row) => row.try_get(0).ok(),
    //         Err(_) => None
    //     }
    // }
}