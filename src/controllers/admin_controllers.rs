use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use crate::models::{self, appstate::AppState, user::{User, UserRole}};

#[axum::debug_handler]
pub async fn create_admin_profile(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAdminProfile>,
) -> impl IntoResponse {
    println!("POST /admin/create_admin_profile");

    let hashed_password = hash_password(&payload.password); // Função que hash a senha (ver abaixo)

    let new_admin = User {
        user_id: None,
        user_name: payload.name.clone(),
        user_email: payload.email.clone(),
        user_password: hashed_password,
        user_role: UserRole::Admin,
        // outros campos, como `user_created_at`, `user_updated_at`, podem ser preenchidos conforme o modelo
    };

    match User::save_user_in_db(&new_admin, state).await {
        Ok(_) => (StatusCode::CREATED, Json(new_admin)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao criar o perfil de administrador").into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateAdminProfile {
    pub name: String,
    pub email: String,
    pub password: String,
}

fn hash_password(password: &str) -> String {
    // Função de hash que pode ser implementada com bcrypt ou outro algoritmo de sua escolha
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}