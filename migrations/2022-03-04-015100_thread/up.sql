CREATE TABLE thread(
    thread_id INTEGER PRIMARY KEY NOT NULL,
    thread_name TEXT NOT NULL,
    lastupdate TIMESTAMP DEFAULT(DATETIME('now','localtime'))
)