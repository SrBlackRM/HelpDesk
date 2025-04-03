use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::{Html, IntoResponse}, Json};

use crate::models::{self, appstate::AppState, ticket::{NewTicket, Ticket}};

#[axum::debug_handler]
pub async fn create_ticket(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewTicket>
) -> impl IntoResponse {
    println!("POST /new_ticket");

    // let client_id = models::user::User::get_id_by_email(payload.ticket_client.user_email.clone()).await.unwrap_or(0);

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
    let template_name: String = String::from("new_ticket.html");

    context.insert("static_path", "/static");

    let rendered: String = state.tera.render(&template_name, &context)
        .expect("Erro ao carregar página");
    Html(rendered)
}