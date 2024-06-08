CREATE TABLE IF NOT EXISTS repository (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  local_path TEXT NOT NULL UNIQUE,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_opened_at DATETIME,
  last_fetched_at DATETIME,
  has_changes BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS state (
  id INTEGER PRIMARY KEY NOT NULL,
  open_repository_id INTEGER,
  FOREIGN KEY (open_repository_id) REFERENCES repository(id)
);

INSERT OR IGNORE INTO state (id, open_repository_id) VALUES (0, NULL);
