DROP TABLE IF EXISTS users;
CREATE TABLE users (
  user_id       VARCHAR(100) NOT NULL,
  display_name  VARCHAR(100),
  hash          VARCHAR(128) NOT NULL,
  login_session VARCHAR(128) NOT NULL,
  PRIMARY KEY (user_id)
);
CREATE INDEX ON users (LOWER(user_id));

DROP TABLE IF EXISTS themes;
CREATE TABLE themes (
  theme_id      SERIAL NOT NULL,
  author        VARCHAR(100) NOT NULL,
  epoch_open    TIMESTAMP NOT NULL,
  theme_text    VARCHAR(255) NOT NULL,
  PRIMARY KEY (theme_id)
);
CREATE INDEX ON themes (author);
CREATE INDEX ON themes (epoch_open);

DROP TABLE IF EXISTS answers;
CREATE TABLE answers (
  user_id       VARCHAR(100) NOT NULL,
  theme_id      INTEGER NOT NULL,
  epoch_submit  TIMESTAMP NOT NULL,
  answer_text   VARCHAR(255) NOT NULL,
  like_count    INTEGER NOT NULL,
  love_count    INTEGER NOT NULL,
  voted         BOOLEAN NOT NULL,
  PRIMARY KEY (user_id, theme_id)
);
CREATE INDEX ON answers (user_id);
CREATE INDEX ON answers (theme_id);

DROP TABLE IF EXISTS login_history;
CREATE TABLE login_history (
  user_id       VARCHAR(100) NOT NULL,
  epoch_login   TIMESTAMP NOT NULL,
  PRIMARY KEY (user_id)
);