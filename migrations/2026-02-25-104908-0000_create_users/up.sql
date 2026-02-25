-- Pastikan extension UUID aktif
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL, 
    full_name VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users (username, email, password_hash, full_name) 
VALUES ('admin', 'admin@example.com', '$2a$12$a3Vp4srEQ2FQ4bRpJCKcaeUfgdYpF/G9YiVsv2eNMrSXAWz2Ly/QK', 'Admin User');
