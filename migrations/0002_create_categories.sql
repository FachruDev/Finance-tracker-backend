-- Categories table
CREATE TABLE IF NOT EXISTS categories (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('income','expense')),
    color TEXT NOT NULL DEFAULT '#888888',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (user_id, name, kind)
);

CREATE INDEX IF NOT EXISTS idx_categories_user ON categories(user_id);

