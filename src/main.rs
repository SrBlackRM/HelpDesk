// std imports
use std::sync::Arc;

// axum imports
use axum::{
    response::{Html, IntoResponse}, 
    routing::get,
    extract::State,
    Router
};

// Tera imports
use tera::Tera;

// SQLX imports
use sqlx::mysql::MySqlPool;

// AppState Struct propria
use models::appstate::AppState;
use tower_http::services::ServeDir;

// modulos proprios imports
mod controllers;
mod routes;
mod models;
mod db;
mod services;

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

    // adiciona o serviço ao app para servir arquivos como js
    .nest_service("/static", ServeDir::new("src/static"))

    // pagina inicial
    .route("/", get(render_index))

    // cria rota para registro de novo usuário / cliente
    .nest("/register", routes::register::RegisterRoute::create_register_route(state.clone()))

    // cria rota para login 
    .nest("/login", routes::auth::AuthRoute::get_authenticated(state.clone()))

    // abrir chamado
    .nest("/new_ticket", routes::ticket::TicketRoute::create_new_ticket_route(state.clone()))

    // listar todos os chamados
    .nest("/tickets", routes::ticket::TicketRoute::list_tickets_route(state.clone()))

    // abrir especificações do chamado
    // .nest("/tickets/[ticket_id]/detail", routes::ticket::TicketRoute::list_tickets_route(state.clone()))
    
    .with_state(state)
    ;
    
    // roda o app escutando na porta 8080
    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .unwrap();
    println!("Servidor rodando em {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.expect("Erro ao iniciar o servidor!");
}


// cria função que renderiza index
async fn render_index(State(state): State<Arc<AppState>>) -> impl IntoResponse{
    println!("GET /index");
    let rendered = state.tera.render("index.html", &tera::Context::new())
        .expect("Erro ao carregar index.html");
    Html(rendered)
}

