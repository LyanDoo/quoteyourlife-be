CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE article_status AS ENUM ('draft', 'published');

CREATE TABLE articles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    excerpt TEXT,
    content JSONB NOT NULL,
    status article_status NOT NULL DEFAULT 'draft',
    author_id UUID NOT NULL,
    published_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Foreign Key (Asumsi nama tabel user kamu adalah 'users')
    CONSTRAINT fk_author 
        FOREIGN KEY(author_id) 
        REFERENCES users(id) 
        ON DELETE CASCADE
);

-- Index untuk slug karena akan sering digunakan untuk pencarian (URL)
CREATE INDEX idx_articles_slug ON articles(slug);

-- Trigger untuk otomatis memperbarui updated_at saat data berubah
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_articles_updated_at
BEFORE UPDATE ON articles
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();