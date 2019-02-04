CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  uuid INT NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  created TIMESTAMP NOT NULL
);

CREATE TABLE topics (
  post_id INT NOT NULL,
  user_id INT NOT NULL,
  name VARCHAR NOT NULL,
  CONSTRAINT PK_topic PRIMARY KEY (post_id, user_id)
);