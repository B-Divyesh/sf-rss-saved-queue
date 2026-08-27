CREATE TABLE IF NOT EXISTS accounts (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  token_hash TEXT NOT NULL UNIQUE,
  created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS saved_items (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  title TEXT NOT NULL,
  url TEXT NOT NULL,
  tags_json TEXT NOT NULL DEFAULT '[]',
  saved_at TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'queue' CHECK(status IN ('queue','read','archived')),
  priority TEXT NOT NULL DEFAULT 'next' CHECK(priority IN ('next','soon','later'))
);
CREATE INDEX IF NOT EXISTS idx_saved_items_account_status ON saved_items(account_id, status, priority, saved_at DESC);

CREATE TABLE IF NOT EXISTS feed_tokens (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  token_hash TEXT NOT NULL UNIQUE,
  label TEXT NOT NULL,
  created_at TEXT NOT NULL,
  revoked_at TEXT
);
CREATE INDEX IF NOT EXISTS idx_feed_tokens_hash ON feed_tokens(token_hash);
