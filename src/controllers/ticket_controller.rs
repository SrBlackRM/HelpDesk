use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::{Html, IntoResponse}, Json};

use crate::models::{appstate::AppState, ticket::{NewTicket, Ticket}};

#[axum::debug_handler]
pub async fn create_ticket(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewTicket>
) -> impl IntoResponse {
    println!("POST /new_ticket");

    // let client_id = models::user::User::get_id_by_email(payload.ticket_client.user_email.clone()).await.unwrap_or(0);

    let new_ticket_post = Ticket::new_ticket(
        payload.ticket_description.clone(),
        payload.ticket_client_id
    );

    println!("{:#?}", &new_ticket_post);
    match Ticket::save_new_ticket_in_db(&new_ticket_post, state).await {
        Ok(_) => (StatusCode::CREATED, Json(new_ticket_post)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao salvar ticket").into_response(),
    }
}

pub async fn list_tickets(
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    println!("GET /tickets");

    let rendered = state.tera.render("tickets.html", &tera::Context::new())
        .expect("Erro ao carregar tickets.html");
    Html(rendered)
}