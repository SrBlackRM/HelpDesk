use std::sync::Arc;
use axum::{routing:: {get, post}, Router};
use crate::{
    controllers::ticket_controller::create_ticket,
    controllers::ticket_controller::list_tickets,
    models::appstate::AppState};

pub struct TicketRoute;

impl TicketRoute{
    pub fn create_new_ticket_route(state: Arc<AppState>) -> Router<Arc<AppState>> {
        Router::new()
        .route("/",post(create_ticket))
        .with_state(state)
    }

    pub fn list_tickets_route(state: Arc<AppState>) -> Router<Arc<AppState>> {
        Router::new()
        .route("/",get(list_tickets))
        .with_state(state)
    }
}