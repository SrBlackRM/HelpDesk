use helpdesk;

select * from Users;

#Dropar o banco de dados (precisa refaze-lo)
drop database helpdesk;

#Apagar todos os dados da tabela Usuários
delete from users where ID_User > 0;

#Resetar auto increment
alter table users auto_increment = 1;

select * from Users;