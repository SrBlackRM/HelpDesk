use std::sync::Arc;
use axum::{routing:: post, Router};
use crate::{controllers::ticket_controller::create_ticket, models::appstate::AppState};

pub struct TicketRoute;

impl TicketRoute{
    pub fn create_new_ticket_route(state: Arc<AppState>) -> Router<Arc<AppState>> {
        Router::new()
        .route("/",post(create_ticket))
        .with_state(state)
    }
}