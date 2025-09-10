-- Transactions table
CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    category_id UUID NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    amount NUMERIC(14,2) NOT NULL,
    occurred_on DATE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_transactions_user_date ON transactions(user_id, occurred_on);
CREATE INDEX IF NOT EXISTS idx_transactions_category ON transactions(category_id);
