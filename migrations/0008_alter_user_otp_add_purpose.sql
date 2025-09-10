ALTER TABLE user_otp_codes
    ADD COLUMN IF NOT EXISTS purpose TEXT NOT NULL DEFAULT 'verify';

-- Optional: backfill is automatic via default

-- Helpful index by purpose
CREATE INDEX IF NOT EXISTS idx_user_otp_purpose ON user_otp_codes(user_id, purpose) WHERE used_at IS NULL;

