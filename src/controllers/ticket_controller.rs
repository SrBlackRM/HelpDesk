use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::{Html, IntoResponse}, Json};
use crate::models::{self, appstate::AppState, ticket::{NewTicket, StatusTicket, Ticket}};

#[axum::debug_handler]
pub async fn create_ticket(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewTicket>
) -> impl IntoResponse {
    println!("POST /new_ticket");

    let new_ticket_post = Ticket::new_ticket(
        payload.ticket_title.clone(),
        payload.ticket_description.clone(),
        payload.ticket_client_id,
        payload.category_id
    );

    println!("{:#?}", &new_ticket_post);
    match Ticket::save_new_ticket_in_db(&new_ticket_post, state).await {
        Ok(_) => (StatusCode::CREATED, Json(new_ticket_post)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao salvar ticket").into_response(),
    }
}

#[axum::debug_handler]
pub async fn update_ticket_status(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateTicketStatus>,
) -> impl IntoResponse {
    println!("POST /update_ticket_status");

    let ticket_id = payload.ticket_id;
    let new_status = match payload.status.as_str() {
        "aberto" => StatusTicket::Aberto,
        "andamento" => StatusTicket::Andamento,
        "fechado" => StatusTicket::Fechado,
        _ => return (StatusCode::BAD_REQUEST, "Status inválido").into_response(),
    };

    match Ticket::update_ticket_status(ticket_id, new_status, state).await {
        Ok(_) => (StatusCode::OK, "Status do ticket atualizado com sucesso").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao atualizar status do ticket").into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateTicketStatus {
    pub ticket_id: i32,
    pub status: String, // aberto, andamento ou fechado
}

#[axum::debug_handler]
pub async fn update_ticket_fields(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateTicketFields>,
) -> impl IntoResponse {
    println!("POST /update_ticket_fields");

    let ticket_id = payload.ticket_id;
    let title = payload.title.clone();
    let description = payload.description.clone();

    match Ticket::update_ticket_fields(ticket_id, title, description, state).await {
        Ok(_) => (StatusCode::OK, "Campos do ticket atualizados com sucesso").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao atualizar campos do ticket").into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateTicketFields {
    pub ticket_id: i32,
    pub title: String,
    pub description: String,
}

pub async fn list_tickets(
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    println!("GET /tickets");

    let context = &mut tera::Context::new();
    let tickets: Vec<Ticket> = models::ticket::Ticket::get_all_tickets(state.clone()).await.expect("Erro");
    
    context.insert("tickets", &tickets);

    let rendered = state.tera.render("tickets.html", &context)
        .expect("Erro ao carregar tickets.html");
    Html(rendered)
}

pub async fn new_ticket(
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    println!("GET /new_ticket");

    let context = &mut tera::Context::new();
    let template_name: &str = "new_ticket.html";

    context.insert("static_path", "/static");

    let rendered: String = state.tera.render(&template_name, &context)
        .expect("Erro ao carregar página");
    Html(rendered)
}