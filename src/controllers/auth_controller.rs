use std::sync::Arc;

use axum::http::StatusCode;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::models::appstate::AppState;
use crate::models::user::User;
use crate::services::auth_service::{jwt_gen, password_verify};

#[derive(Debug, Deserialize)]
pub struct LoginData{
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse{
    pub token: String,
}

#[derive(Serialize)]
pub struct ErrorMessage {
    error: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>, 
    Json(data): Json<LoginData>)
    ->  Result<Json<TokenResponse>, (StatusCode, Json<ErrorMessage>)> {
        
        println!("POST /login");

        let user = User::get_user_by_email(data.email, State(state)).await;
        println!("{:?}", user);

        if let Ok(user) = user {
            if password_verify(&data.password, &user.user_password){
                let token = jwt_gen(user.user_id.unwrap());
                println!("Usuário logado com sucesso\nToken: {:?}", token);
                return Ok(Json(TokenResponse { token }));
            }
        }

        Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorMessage {
                error: "Credenciais inválidas".to_string(),
            }),
        ))
}