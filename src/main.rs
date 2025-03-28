// std imports
use std::sync::Arc;

// axum imports
use axum::{
    response::{Html, IntoResponse}, 
    routing::{get, post},
    extract::State,
    Router
};

// Tera imports
use tera::Tera;

// SQLX imports
use sqlx::mysql::MySqlPool;

// AppState Struct propria
use models::appstate::AppState;

// modulos proprios imports
mod controllers;
mod models;
mod db;

// tokio para tornar a função main asyncrona
#[tokio::main]
async fn main() {
    // tera é usado para renderizar template, como django
    let tera = Tera::new("./src/templates/**/*.html").expect("Erro ao carregar templates");

    // estabelece conexão com banco
    let pool: MySqlPool = db::connection::establish_connection()
        .await
        .expect("Erro ao estabelecer conexão");

    // Cria a struct com os estados
    let state = Arc::new(AppState {tera, pool});

    // cria rota raiz do servidor
    let app = Router::new()
    .route("/", get(render_index))

    // cria rota para registro de novo usuário / cliente
    .route("/register", post(controllers::user_controller::create_user))
    .with_state(state)
    ;
    
    // roda o app escutando na porta 8080
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("Servidor rodando em {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.expect("Erro ao iniciar o servidor!");
}


// cria função que renderiza index
async fn render_index(State(state): State<Arc<AppState>>) -> impl IntoResponse{
    let rendered = state.tera.render("index.html", &tera::Context::new())
        .expect("Erro ao carregar index.html");
    Html(rendered)
}