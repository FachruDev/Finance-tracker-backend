CREATE TABLE IF NOT EXISTS user_otp_codes (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    code TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_user_otp_user ON user_otp_codes(user_id);
CREATE INDEX IF NOT EXISTS idx_user_otp_valid ON user_otp_codes(user_id, expires_at) WHERE used_at IS NULL;

