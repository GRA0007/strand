CREATE TABLE IF NOT EXISTS repository (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  local_path TEXT NOT NULL UNIQUE,
  created_at TEXT NOT NULL,
  last_opened_at TEXT,
  last_fetched_at TEXT,
  has_changes BOOLEAN
);

CREATE TABLE IF NOT EXISTS state (
  id INTEGER PRIMARY KEY NOT NULL,
  open_repository INTEGER,
  FOREIGN KEY (open_repository) REFERENCES repository(id)
);

INSERT OR IGNORE INTO state (id, open_repository) VALUES (0, NULL);
