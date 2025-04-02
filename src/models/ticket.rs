use std::sync::Arc;

use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, query, Row};

use super::appstate::AppState;

#[derive(Debug,Deserialize, Serialize)]
pub struct Ticket{
    pub ticket_id: Option<i32>,
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
            ticket_id: None,
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

    pub async fn get_all_tickets(state: Arc<AppState>) -> Result<Vec<Ticket>, sqlx::Error> {
        let rows = query(
            "SELECT 
                ID_Ticket,
                Ticket_Title,
                Ticket_Status,
                Ticket_Priority,
                ID_Category,
                Ticket_Description,
                ID_User_Technical,
                ID_User_Requesting,
                Ticket_Opening_Data,
                Ticket_Closing_Data            
            FROM Tickets"
        )
        .fetch_all(&state.pool)
        .await?; // Propaga erro automaticamente
    
        let tickets: Vec<Ticket> = rows.into_iter().map(|row| Ticket {
            ticket_id: row.try_get("ID_Ticket").unwrap_or(Some(0)),
            ticket_title: row.try_get("Ticket_Title").unwrap_or_else(|_| "".to_string()),
            ticket_status: row.try_get("Ticket_Status").unwrap_or_else(|_| StatusTicket::Aberto),
            ticket_priority: row.try_get("Ticket_Priority").unwrap_or_else(|_| Priority::Média),
            ticket_category: row.try_get("ID_Category").unwrap_or(0),
            ticket_description: row.try_get("Ticket_Description").unwrap_or_else(|_| "".to_string()),
            ticket_client_id: row.try_get("ID_User_Requesting").unwrap_or(0),
            ticket_technical: row.try_get("ID_User_Technical").ok(),
            ticket_closing_data: row.try_get("Ticket_Closing_Data").ok(), 
            ticket_opening_data: row.try_get("Ticket_Opening_Data").unwrap_or(Utc::now()),
        }).collect();
    
        Ok(tickets)
    }

    
}