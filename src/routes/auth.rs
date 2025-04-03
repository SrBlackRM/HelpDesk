use std::sync::Arc;

use axum::{
    extract::State, response::{Html, IntoResponse}, routing::{get, post}, Router
};

use crate::{controllers, models::appstate::AppState};

pub struct AuthRoute;

impl AuthRoute {
    pub fn get_authenticated(state: Arc<AppState>)  -> Router<Arc<AppState>>{
        Router::new()
        .route("/", get(render_login))
        .route("/", post(controllers::auth_controller::login))
        .with_state(state)
    }   
}


// renderiza a pagina de login metodo GET
pub async fn render_login(State(state): State<Arc<AppState>>) -> impl IntoResponse{
    println!("GET /login");

    let template_name: &str = "login.html";
    let context = &mut tera::Context::new();
    
    context.insert("static_path", "/static");

    let rendered = state.tera.render(&template_name, &context)
        .expect("Erro ao carregar login.html");
    Html(rendered)
}