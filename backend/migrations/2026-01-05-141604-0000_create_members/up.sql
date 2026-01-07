-- Your SQL goes here
CREATE TABLE member (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    f_name TEXT NOT NULL,
    m_name TEXT NOT NULL,
    l_name TEXT NOT NULL,
    email TEXT NOT NULL,
    address TEXT NOT NULL,
    age INTEGER NOT NULL
);

-- Record of each payment
CREATE TABLE contributions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    member_id INTEGER NOT NULL,
    amount_paid REAL NOT NULL, -- SQLite uses REAL for decimals
    month_period TEXT NOT NULL, -- Format: YYYY-MM-DD
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(member_id) REFERENCES member(id)
);

-- Global settings for the admin
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- Set default monthly fee
INSERT INTO settings (key, value) VALUES ('monthly_fee', '50.00');