use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
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
}