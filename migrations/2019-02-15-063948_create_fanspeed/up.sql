-- Your SQL goes here
CREATE TABLE fanspeed (
    id SERIAL PRIMARY KEY,
    timestamp timestamp NOT NULL,
    fan_name TEXT NOT NULL,
    rpm INTEGER NOT NULL
);
