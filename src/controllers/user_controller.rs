use axum::{
    Json, 
    extract::State
};
use std::sync::Arc;
use sqlx::query;
use crate::models::{appstate::AppState, user::User};

// criar novo usuário e inserir no banco
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<User>
) -> Json<User> {
    // utiliza a implementação build_user para criar novo usuário
    let new_user = User::build_user(
        payload.user_name,
        payload.user_email,
        payload.user_password,
    );

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

    Json(new_user)
}