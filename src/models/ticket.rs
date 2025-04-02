use std::sync::Arc;

use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, query};

use super::appstate::AppState;

#[derive(Debug,Deserialize, Serialize)]
pub struct Ticket{
    pub ticket_opening_data: DateTime<Utc>,
    pub ticket_closing_data: Option<DateTime<Utc>>,
    pub ticket_status: StatusTicket,
    pub ticket_priority: Priority,
    pub ticket_description: String,
    pub ticket_technical: Option<i32>,
    pub ticket_title: String,
    pub ticket_client_id: i32,
    pub ticket_category: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTicket{
    pub ticket_description: String,
    pub ticket_client_id: i32,
    pub ticket_title: String,
    pub category_id: i32
}


#[derive(Debug, Deserialize, Serialize, Type)]
#[sqlx(type_name = "ENUM('Aberto', 'Andamento', 'Fechado')", rename_all = "lowercase")]
pub enum StatusTicket{
    Aberto,
    Andamento,
    Fechado
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[sqlx(type_name = "ENUM('baixa', 'média', 'alta')", rename_all = "lowercase")]
pub enum Priority{
    Baixa,
    Média,
    Alta
}

impl Ticket {
    pub fn new_ticket(title: String, description: String, client_id: i32, category: i32) -> Self{
        Self { 
            ticket_opening_data: Local::now().into(), 
            ticket_closing_data: None, 
            ticket_status: StatusTicket::Aberto, 
            ticket_priority: Priority::Média, 
            ticket_description: description, 
            ticket_technical: None, 
            ticket_client_id: client_id,
            ticket_title: title,
            ticket_category: category
        }
    }

    pub async fn save_new_ticket_in_db(new_ticket_post: &Self, state: Arc<AppState>)  -> Result<(), sqlx::Error> {
        // Inserir no banco com a query corrigida 
        query(
            "INSERT INTO Tickets (
                Ticket_Title,
                Ticket_Description, 
                ID_User_Requesting,
                ID_Category
                ) VALUES (?, ?, ?, ?)"
        )
        .bind(&new_ticket_post.ticket_title)
        .bind(&new_ticket_post.ticket_description)
        .bind(&new_ticket_post.ticket_client_id)
        .bind(&new_ticket_post.ticket_category)
        .execute(&state.pool)
        .await?;

        Ok(())
    }
}