use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Form};

use crate::{models::{appstate::AppState, user::User}, utils::misc::password_md5_hasher};


// criar novo usuário e inserir no banco
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Form(ref payload): Form<User>
) -> impl IntoResponse {
    println!("POST /register.html");
    // utiliza a implementação build_user para criar novo usuário
    let new_user = User::build_user(
        payload.user_name.clone(),
        payload.user_email.clone(),
        password_md5_hasher(payload.user_password.clone().as_str()),
    );

    // utiliza a implementação save user para salvar no banco
    User::save_user_in_db(&new_user, state).await;

    println!("Recebido: {:?}", payload);
    "Usuário recebido com sucesso!"
}