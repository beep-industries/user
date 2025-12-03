-- Users table
CREATE TABLE IF NOT EXISTS users (
    sub UUID PRIMARY KEY,
    display_name VARCHAR(255) NOT NULL DEFAULT '',
    profile_picture VARCHAR(500) NOT NULL DEFAULT '',
    description VARCHAR(255) NOT NULL DEFAULT '',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Param table
CREATE TABLE IF NOT EXISTS param (
    sub UUID PRIMARY KEY REFERENCES users(sub) ON DELETE CASCADE,
    theme VARCHAR(50) DEFAULT 'light',
    lang VARCHAR(10) DEFAULT 'en',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes
CREATE INDEX idx_users_display_name ON users(display_name);
