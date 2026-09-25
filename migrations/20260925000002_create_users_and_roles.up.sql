-- Migrasi 02: Klaster Identitas & Otorisasi Pengguna
-- Tabel: users

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255),
    display_name VARCHAR(100) NOT NULL,
    role VARCHAR(20) NOT NULL DEFAULT 'reader' CHECK (role IN ('reader', 'admin')),
    avatar_url TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indeks Otentikasi dan Otorisasi
CREATE INDEX IF NOT EXISTS idx_users_email ON users USING btree (email);
CREATE INDEX IF NOT EXISTS idx_users_admin_role ON users USING btree (role) WHERE role = 'admin';
