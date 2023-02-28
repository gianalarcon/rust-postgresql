use std::env;

pub const CREATE_DB: &str = "
        CREATE TABLE IF NOT EXISTS users (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR UNIQUE NOT NULL,
            password        VARCHAR NOT NULL,
            email           VARCHAR UNIQUE NOT NULL
        )";
pub const INSERT_USER: &str = "INSERT INTO users (username, password, email) VALUES ($1, $2, $3)";
pub const UPDATE_USER: &str = "UPDATE users SET username=$2, password=$3, email=$4 WHERE id=$1";
pub const DELETE_USER: &str = "DELETE FROM users WHERE id=$1";
pub const SELECT_ALL_USERS: &str = "SELECT username, password, email FROM users";

pub const DB_URL: &str = env!("DATABASE_URL");
