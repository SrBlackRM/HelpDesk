use std::sync::Arc;

use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, query, Row};

use super::appstate::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct Ticket {
    pub ticket_id: Option<i32>,
    pub ticket_opening_data: DateTime<Utc>,
    pub ticket_closing_data: Option<DateTime<Utc>>,
    pub ticket_status: StatusTicket,
    pub ticket_priority: Priority,
    pub ticket_description: String,
    pub ticket_technical: Option<i32>,
    pub ticket_title: String,
    pub ticket_client_id: i32,
    pub ticket_category: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTicket {
    pub ticket_description: String,
    pub ticket_client_id: i32,
    pub ticket_title: String,
    pub category_id: i32,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[sqlx(type_name = "ENUM('Aberto', 'Andamento', 'Fechado')", rename_all = "lowercase")]
pub enum StatusTicket {
    Aberto,
    Andamento,
    Fechado,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[sqlx(type_name = "ENUM('baixa', 'média', 'alta')", rename_all = "lowercase")]
pub enum Priority {
    Baixa,
    Média,
    Alta,
}

impl Ticket {
    pub fn new_ticket(title: String, description: String, client_id: i32, category: i32) -> Self {
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
            ticket_category: category,
        }
    }

    pub async fn save_new_ticket_in_db(new_ticket_post: &Self, state: Arc<AppState>) -> Result<(), sqlx::Error> {
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
        .await?;

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

    // Função para atualizar o status de um ticket
    pub async fn update_ticket_status(ticket_id: i32, new_status: StatusTicket, state: Arc<AppState>) -> Result<(), sqlx::Error> {
        let query_str = match new_status {
            StatusTicket::Fechado => "UPDATE Tickets SET Ticket_Status = ?, Ticket_Closing_Data = ? WHERE ID_Ticket = ?",
            _ => "UPDATE Tickets SET Ticket_Status = ? WHERE ID_Ticket = ?",
        };

        let closing_time = if let StatusTicket::Fechado = new_status {
            Some(Utc::now())
        } else {
            None
        };

        query(query_str)
            .bind(new_status)
            .bind(closing_time)
            .bind(ticket_id)
            .execute(&state.pool)
            .await?;

        Ok(())
    }

    // Usado para mudar a prioridade de um ticket
    pub async fn update_ticket_priority(ticket_id: i32, new_priority: Priority, state: Arc<AppState>) -> Result<(), sqlx::Error> {
        query(
            "UPDATE Tickets SET Ticket_Priority = ? WHERE ID_Ticket = ?"
        )
        .bind(new_priority)
        .bind(ticket_id)
        .execute(&state.pool)
        .await?;

        Ok(())
    }

    // usado para mudar descrição e titulo de um ticket 
    pub async fn update_ticket_fields(ticket_id: i32, title: String, description: String, state: Arc<AppState>) -> Result<(), sqlx::Error> {
        query(
            "UPDATE Tickets SET Ticket_Title = ?, Ticket_Description = ? WHERE ID_Ticket = ?"
        )
        .bind(title)
        .bind(description)
        .bind(ticket_id)
        .execute(&state.pool)
        .await?;

        Ok(())
    }
}

impl Ticket {
    //Buscar ticket por id
    pub async fn get_ticket_by_id(ticket_id: i32, state: Arc<AppState>) -> Result<Ticket, sqlx::Error> {
        let row = query(
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
            FROM Tickets
            WHERE ID_Ticket = ?"
        )
        .bind(ticket_id)
        .fetch_one(&state.pool)
        .await?;

        let ticket = Ticket {
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
        };

        Ok(ticket)
    }
}