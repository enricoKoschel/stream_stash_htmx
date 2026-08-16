CREATE TABLE user_sessions (
    session_id TEXT NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE tower_sessions (
    id TEXT PRIMARY KEY NOT NULL,
    data BLOB NOT NULL,
    expiry_date INTEGER NOT NULL
);

CREATE TRIGGER user_sessions_cleanup_after_tower_sessions_delete
AFTER DELETE ON tower_sessions
FOR EACH ROW
BEGIN
    DELETE FROM user_sessions
    WHERE session_id = OLD.id;
END;
