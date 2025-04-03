use axum::extract::State;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::Row;
use sqlx::Type;
use sqlx::query;
use crate::models::appstate::AppState;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    #[sqlx(rename = "ID_User")] 
    pub user_id: Option<i32>,
    #[sqlx(rename = "User_Name")]
    pub user_name: String,
    #[sqlx(rename = "User_Email")]
    pub user_email: String,
    #[sqlx(rename = "User_Password")]
    pub user_password: String,
    #[sqlx(rename = "User_Phone")]
    pub user_phone: Option<String>,
    #[sqlx(rename = "User_Section")]
    pub user_section: Option<String>,
    #[sqlx(rename = "User_Role")]
    pub user_role: UserRole,
    #[sqlx(rename = "User_Expertise")]
    pub user_expertise: Option<String>,
    #[sqlx(rename = "User_Active")]
    pub user_active: bool
}

#[derive(Debug, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "User_Role", rename_all = "lowercase")]
pub enum UserRole {
    Cliente,
    Tecnico,
    Administrador,
}

impl FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cliente" => Ok(UserRole::Cliente),
            "tecnico" => Ok(UserRole::Tecnico),
            "administrador" => Ok(UserRole::Administrador),
            _ => Err(format!("Valor inválido para UserRole: {}", s)),
        }
    }
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
            "INSERT INTO Users (User_Name, User_Email, User_Password) VALUES (?, ?, ?)"
        )
        .bind(&new_user.user_name)
        .bind(&new_user.user_email)
        .bind(&new_user.user_password)
        .execute(&state.pool)
        .await
        .expect("Erro ao inserir usuário");
    }

    pub async fn get_user_by_email(email: String, state: State<Arc<AppState>>) -> Result<User, sqlx::Error> {
        let row = query(
            "SELECT 
                ID_User,
                User_Name,
                User_Email,
                User_Password,
                User_Phone,
                User_Section,
                User_Role,
                User_Expertise,
                User_Active          
            FROM Users WHERE User_Email = ?"
        )
        .bind(email)
        .fetch_one(&state.pool)
        .await?; // Propaga erro automaticamente
    
        let user  = User {
            user_id: row.try_get("ID_User").unwrap(),
            user_name: row.try_get("User_Name").unwrap(),
            user_email: row.try_get("User_Email").unwrap(),
            user_password: row.try_get("User_Password").unwrap(),
            user_phone: row.try_get("User_Phone").ok(),
            user_section: row.try_get("User_Section").ok(),
            user_role: row.try_get::<String, _>("User_Role")
                .ok()
                .and_then(|role| UserRole::from_str(&role).ok()) // Converte String para UserRole
                .unwrap_or(UserRole::Cliente), // Default para Cliente
            user_expertise: row.try_get("User_Expertise").ok(),
            user_active: row.try_get("User_Active").unwrap()
        };

        Ok(user)
    }
}