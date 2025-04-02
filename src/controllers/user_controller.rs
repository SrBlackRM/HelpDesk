use axum::{
    Json, 
    extract::State
};

use std::sync::Arc;

use crate::models::{
    appstate::AppState, 
    user::User};

use crate::utils::misc::password_md5_hasher;

// criar novo usuário e inserir no banco
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<User>
) -> Json<User> {
    println!("POST /register.html");
    // utiliza a implementação build_user para criar novo usuário
    let new_user = User::build_user(
        payload.user_name,
        payload.user_email,
        password_md5_hasher(payload.user_password.as_str()),
    );

    // utiliza a implementação save user para salvar no banco
    User::save_user_in_db(&new_user, state).await;

    Json(new_user)
}