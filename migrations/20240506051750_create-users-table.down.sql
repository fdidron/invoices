-- Add down migration script here
DROP INDEX idx_user_email ON users(email);
DROP TABLE IF EXISTS users;
