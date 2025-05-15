use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Form};
use serde::{Deserialize, Serialize};

use crate::{models::{appstate::AppState, user::User}, services::auth_service::password_hash};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser{
    user_name: String,
    user_email: String,
    user_password: String
}


pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Form(ref payload): Form<NewUser>
) -> impl IntoResponse {
    println!("POST /register.html");
    
    let new_user = User::build_user(
        payload.user_name.clone(),
        payload.user_email.clone(),
        password_hash(payload.user_password.clone().as_str()),
    );

    
    User::save_user_in_db(&new_user, state).await;

    println!("Recebido: {:?}", payload);
    "Usuário recebido com sucesso!"
}