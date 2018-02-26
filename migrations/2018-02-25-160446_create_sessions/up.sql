CREATE TABLE sessions (
  key VARCHAR PRIMARY KEY,
  user_id SERIAL REFERENCES users(id)
);
