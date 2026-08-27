CREATE TABLE IF NOT EXISTS feeds (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  url TEXT NOT NULL UNIQUE,
  title TEXT NOT NULL,
  created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS items (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  feed_id INTEGER REFERENCES feeds(id) ON DELETE CASCADE,
  title TEXT NOT NULL,
  url TEXT NOT NULL UNIQUE,
  source TEXT NOT NULL,
  summary TEXT NOT NULL DEFAULT '',
  published_at TEXT,
  saved_at TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'queue' CHECK(status IN ('queue','read','archived')),
  priority TEXT NOT NULL DEFAULT 'next' CHECK(priority IN ('next','soon','later')),
  content_hash TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_items_status_priority ON items(status, priority, saved_at DESC);
