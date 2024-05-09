CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    otp_secret VARCHAR(255) NOT NULL,
    profile_pic_url VARCHAR(255),
    street VARCHAR(255),
    city VARCHAR(255),
    postal_code VARCHAR(16),
    country_code VARCHAR(2),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_user_email ON users(email);

