use helpdesk;

#SELECTS
select * from Users;
select * from Tickets;
select * from Categories;

#INSERT INTO PARA TESTE
insert into Categories (Category_Name) value ("Impressora");
insert into Categories (Category_Name) value ("Computador");

#Dropar o banco de dados (precisa refaze-lo)
drop database helpdesk;

#Apagar todos os dados da tabela Usuários
delete from tickets where ID_Ticket > 0;

#Resetar auto increment
alter table tickets auto_increment = 1;

#INSERT INTO PARA TESTE TAMBÉM
INSERT INTO Tickets (Ticket_Title, Ticket_Description, ID_User_Requesting, ID_Category) VALUES ("Computador não liga", "Meu computador não liga, já tentei outras tomadas, já tentei trocar de monitor e nada funciona", 2, 1);


update Tickets set ticket_title = "Impressora" where ID_Ticket = 1;