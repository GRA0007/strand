CREATE TABLE IF NOT EXISTS repositories (
  id INTEGER PRIMARY KEY NOT NULL,
  folder_name TEXT NOT NULL,
  local_path TEXT NOT NULL UNIQUE,
  created_at TEXT NOT NULL,
  last_fetched_at TEXT
);
