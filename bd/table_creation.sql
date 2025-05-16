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
    ID_User INT PRIMARY KEY AUTO_INCREMENT,
    User_Email VARCHAR(255) UNIQUE NOT NULL,
    User_Password VARCHAR(255) NOT NULL,
    User_Role ENUM('user', 'adm') NOT NULL
);

CREATE TABLE Tickets (
    ID_Ticket INT PRIMARY KEY AUTO_INCREMENT,
    Ticket_Title VARCHAR(255) NOT NULL,
    Ticket_Description TEXT NOT NULL,
    Ticket_Category ENUM('Software', 'Hardware', 'Redes', 'Acesso') NOT NULL,
    Ticket_Priority ENUM('baixa', 'media', 'alta') DEFAULT NULL,
    ID_User INT NOT NULL,
    Ticket_Status ENUM('aberto', 'fechado') NOT NULL DEFAULT 'aberto',
    FOREIGN KEY (ID_User) REFERENCES Users(ID_User)
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

#ALTER TABLE Tickets ADD ID_Category INT NOT NULL;
#SET FOREIGN_KEY_CHECKS = 1;
#ALTER TABLE Tickets ADD CONSTRAINT FK_Category FOREIGN KEY (ID_Category) REFERENCES Categories(ID_Category) ON DELETE CASCADE;
#ALTER TABLE Tickets ADD Ticket_Title VARCHAR(250) NOT NULL;
#ALTER TABLE Tickets DROP COLUMN  Ticket_Title;