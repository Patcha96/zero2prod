-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
'2c9797eb-be93-4d4b-9cdc-723f15f170b8', 
'admin',
'$argon2id$v=19$m=15000,t=2,p=1$gDhBfoVTB3FgbFjqH+GUcQ$8fQ9wIsKi8bGFff3OKeMskSsouDI848LWwN+gOG9iqQ'
);