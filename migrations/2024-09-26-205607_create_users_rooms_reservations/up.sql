-- Your SQL goes here
-- Création de la table users
CREATE TABLE users (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    role VARCHAR NOT NULL
);

-- Création de la table rooms
CREATE TABLE rooms (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    capacity INTEGER NOT NULL,
    equipments TEXT[] NOT NULL,
    location VARCHAR NOT NULL
);

-- Création de la table reservations
CREATE TABLE reservations (
    id UUID PRIMARY KEY,
    room_id UUID NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    priority INTEGER NOT NULL,
    status VARCHAR NOT NULL
);

-- Index pour améliorer les performances des requêtes
CREATE INDEX idx_reservations_room_id ON reservations (room_id);
CREATE INDEX idx_reservations_user_id ON reservations (user_id);
CREATE INDEX idx_reservations_start_time ON reservations (start_time);
CREATE INDEX idx_reservations_end_time ON reservations (end_time);
