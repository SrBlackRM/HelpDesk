show databases;

create database helpdesk;

use helpdesk;

/* Lógico_1: */

CREATE TABLE Categories (
    ID_Category INT NOT NULL AUTO_INCREMENT,
    Category_Name VARCHAR(50) NOT NULL,
    
    PRIMARY KEY (ID_Category)
);

CREATE TABLE Users (
    ID_User INT NOT NULL AUTO_INCREMENT,
    User_Name VARCHAR(100) NOT NULL,
    User_Email VARCHAR(150) NOT NULL UNIQUE,
    User_Password VARCHAR(150) NOT NULL,
    User_Phone VARCHAR(20),
    User_Section VARCHAR(50),
    User_Role ENUM('Cliente', 'Técnico', 'Administrador') NOT NULL DEFAULT 'Cliente',
    User_Expertise VARCHAR(100),
    User_Active BOOL NOT NULL DEFAULT FALSE,
    
    PRIMARY KEY (ID_User)
);

CREATE TABLE Tickets (
    ID_Ticket INT NOT NULL AUTO_INCREMENT,
    Ticket_Opening_Data DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    Ticket_Closing_Data DATETIME,
    Ticket_Status ENUM('Aberto', 'Em andamento', 'Fechado') NOT NULL DEFAULT 'Aberto',
    Ticket_Priority ENUM('Baixa', 'Média', 'Alta') NOT NULL DEFAULT 'Média',
    Ticket_Description TEXT NOT NULL,
    ID_User_Technical INT, -- FK
    ID_User_Requesting INT NOT NULL, -- FK
    
    PRIMARY KEY (ID_Ticket),
    
    CONSTRAINT FK_UserTechnical FOREIGN KEY (ID_User_Technical)
    REFERENCES Users(ID_User),
    
    CONSTRAINT FK_UserRequesting FOREIGN KEY (ID_User_Requesting)
    REFERENCES Users(ID_User)
);

CREATE TABLE Interactions (
    ID_Interaction INT NOT NULL AUTO_INCREMENT,
    ID_Ticket INT NOT NULL, -- FK
    ID_User INT NOT NULL, -- FK
    Ticket_Message TEXT NOT NULL,
    Ticket_Data DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    Interaction_Feedback INT,
    Message_Feedback TEXT,
    
    PRIMARY KEY (ID_Interaction),
    
    CONSTRAINT FK_TicketInteraction FOREIGN KEY (ID_Ticket) 
    REFERENCES Tickets(ID_Ticket) ON DELETE CASCADE,
    
    CONSTRAINT FK_UserInteraction FOREIGN KEY (ID_User) 
    REFERENCES Users(ID_User) ON DELETE CASCADE
);

ALTER TABLE Users ADD User_Active BOOL NOT NULL DEFAULT FALSE;