CREATE TABLE IF NOT EXISTS nodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    replication_from INTEGER,
    replication_type TEXT NOT NULL,
    host TEXT NOT NULL,
    port INTEGER NOT NULL,
    FOREIGN KEY (parent_id) REFERENCES nodes (id)
);

CREATE TABLE IF NOT EXISTS clusters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

);