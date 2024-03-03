-- Add up migration script here

CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(50) PRIMARY KEY NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    about TEXT DEFAULT 'A user at InkLink' NOT NULL ,
    account_status VARCHAR(20) DEFAULT 'active' NOT NULL,
    registration_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL ,
    last_login_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
