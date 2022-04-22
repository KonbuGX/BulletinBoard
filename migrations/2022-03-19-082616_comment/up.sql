CREATE TABLE thread_comment(
    thread_id INTEGER NOT NULL,
    comment_no INTEGER NOT NULL,
    comment_name TEXT NOT NULL,
    comment TEXT NOT NULL,
    lastupdate TIMESTAMP DEFAULT(DATETIME('now','localtime')),
    PRIMARY KEY (thread_id, comment_no)
)