use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use crate::models::{self, appstate::AppState, ticket::{Ticket, Priority}, user::{User, UserRole}};

// usado para criar um perfil de administrador
#[axum::debug_handler]
pub async fn create_admin_profile(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAdminProfile>,
) -> impl IntoResponse {
    println!("POST /admin/create_admin_profile");

    let hashed_password = hash_password(&payload.password); 

    let new_admin = User {
        user_id: None,
        user_name: payload.name.clone(),
        user_email: payload.email.clone(),
        user_password: hashed_password,
        user_role: UserRole::Admin,
    };

    match User::save_user_in_db(&new_admin, state).await {
        Ok(_) => (StatusCode::CREATED, Json(new_admin)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao criar o perfil de administrador").into_response(),
    }
}

// usado para alterar a prioridade de um ticket
#[axum::debug_handler]
pub async fn update_ticket_priority(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateTicketPriority>,
) -> impl IntoResponse {
    println!("PATCH /admin/update_ticket_priority");

    
    let mut ticket = match Ticket::get_ticket_by_id(payload.ticket_id, state.clone()).await {
        Ok(ticket) => ticket,
        Err(_) => return (StatusCode::NOT_FOUND, "Ticket não encontrado").into_response(),
    };

    
    ticket.ticket_priority = payload.new_priority;

   
    match Ticket::update_ticket_priority_in_db(&ticket, state.clone()).await {
        Ok(_) => (StatusCode::OK, Json(ticket)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao atualizar a prioridade do ticket").into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateTicketPriority {
    pub ticket_id: i32,
    pub new_priority: Priority,
}

fn hash_password(password: &str) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}

//teste para ver se o commit vai