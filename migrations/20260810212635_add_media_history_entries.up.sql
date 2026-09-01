CREATE TABLE media_history_entries (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    media_id INTEGER NOT NULL,
    media_type TEXT NOT NULL,
    rating INTEGER CHECK (rating BETWEEN 1 AND 10),
    title TEXT,
    comment TEXT,
    start_date DATE,
    end_date DATE,
    FOREIGN KEY(user_id) REFERENCES users(id),
    FOREIGN KEY(media_id, media_type, user_id) REFERENCES media(id, type, user_id)
);
