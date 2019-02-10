CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  uuid INT NOT NULL,
  title VARCHAR NOT NULL,
  synopsis TEXT,
  body TEXT NOT NULL,
  created TIMESTAMP NOT NULL
);

CREATE TABLE topics (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE post_topics (
  post_id INT NOT NULL references posts(id),
  topic_id INT NOT NULL references topics(id),
  CONSTRAINT PK_topic PRIMARY KEY (post_id, topic_id)
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  uuid INT NOT NULL,
  name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  password_hash BIGINT NOT NULL
);

CREATE TABLE user_topics (
  user_id INT NOT NULL references users(id),
  topic_id INT NOT NULL references topics(id),
  CONSTRAINT PK_u_t PRIMARY KEY (user_id, topic_id)
);

CREATE TABLE user_posts (
  user_id INT NOT NULL references users(id),
  post_id INT NOT NULL references posts(id),
  CONSTRAINT PK_u_p PRIMARY KEY (user_id, post_id)
);
