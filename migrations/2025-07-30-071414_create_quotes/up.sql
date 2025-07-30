-- Your SQL goes here
-- up.sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE quotes (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  text VARCHAR NOT NULL,
  author VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);