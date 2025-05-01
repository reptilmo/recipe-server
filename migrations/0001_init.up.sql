-- Add up migration script here
CREATE TABLE IF NOT EXISTS recipes (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  preparation TEXT NOT NULL,
  source TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ingredients (
  recipe_id INTEGER NOT NULL,
  ingredient TEXT NOT NULL,
  FOREIGN KEY (recipe_id) REFERENCES recipes(id)
);

CREATE TABLE IF NOT EXISTS tags (
  recipe_id INTEGER NOT NULL,
  tag VARCHAR(200) NOT NULL,
  FOREIGN KEY (recipe_id) REFERENCES recipes(id)
);
