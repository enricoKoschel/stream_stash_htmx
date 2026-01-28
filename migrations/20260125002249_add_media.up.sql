CREATE TABLE media (
    id INTEGER NOT NULL,
    type TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    state TEXT NOT NULL,
    PRIMARY KEY(id, type, user_id),
    FOREIGN KEY(user_id) REFERENCES users(id)
);
