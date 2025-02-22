-- Create URLs table
CREATE TABLE IF NOT EXISTS urls (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    original_url TEXT NOT NULL,
    short_code VARCHAR(10) NOT NULL UNIQUE,
    visits BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMPTZ
);

-- Create index on short_code for faster lookups
CREATE INDEX IF NOT EXISTS idx_urls_short_code ON urls(short_code);

-- Create index on visits for stats queries
CREATE INDEX IF NOT EXISTS idx_urls_visits ON urls(visits DESC);

-- Create index on expiration for cleanup
CREATE INDEX IF NOT EXISTS idx_urls_expires_at ON urls(expires_at)
    WHERE expires_at IS NOT NULL;
