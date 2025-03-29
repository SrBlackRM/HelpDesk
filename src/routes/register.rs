use axum::{
    routing::{get, post},
    extract::State,
    response::IntoResponse,
    Router,
};

use crate::Html;

use crate::controllers;
use crate::models::appstate::AppState;
use std::sync::Arc;

pub struct RegisterRoute;

// implemento uma rota para lidar com get e post
impl RegisterRoute {
    // traz o state do main e retorna a rota
    pub fn create_register_route(state: Arc<AppState>) -> Router<Arc<AppState>> {
            Router::new()
            .route("/", get(render_register))
            .route("/", post(controllers::user_controller::create_user))
            .with_state(state)
    }
}

// renderiza a pagina de registro metodo GET
pub async fn render_register(State(state): State<Arc<AppState>>) -> impl IntoResponse{
    println!("GET /register.html");
    let rendered = state.tera.render("register.html", &tera::Context::new())
        .expect("Erro ao carregar register.html");
    Html(rendered)
}