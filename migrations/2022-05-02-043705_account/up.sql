CREATE TABLE account(
    account_no INTEGER PRIMARY KEY NOT NULL,
    account_name TEXT NOT NULL,
    password TEXT NOT NULL,
    lastupdate TIMESTAMP DEFAULT(DATETIME('now','localtime'))
)