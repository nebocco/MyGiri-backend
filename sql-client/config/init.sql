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
  id            SERIAL,
  user_id       VARCHAR(100) NOT NULL,
  theme_id      INTEGER NOT NULL,
  epoch_submit  TIMESTAMP NOT NULL,
  answer_text   VARCHAR(255) NOT NULL,
  score         INTEGER NOT NULL,
  voted         BOOLEAN NOT NULL,
  PRIMARY KEY (user_id, theme_id)
);
CREATE INDEX ON answers (id);
CREATE INDEX ON answers (user_id);
CREATE INDEX ON answers (theme_id);

DROP TABLE IF EXISTS login_history;
CREATE TABLE login_history (
  user_id       VARCHAR(100) NOT NULL,
  epoch_login   TIMESTAMP NOT NULL,
  PRIMARY KEY (user_id)
);

DROP TABLE IF EXISTS votes;
CREATE TABLE votes (
  user_id      VARCHAR(100) NOT NULL,
  theme_id     INTEGER NOT NULL,
  answer_id    INTEGER NOT NULL,
  score        INTEGER NOT NULL,
  PRIMARY KEY (user_id, theme_id, answer_id)
);
CREATE INDEX ON votes (user_id, theme_id);
CREATE INDEX ON votes (theme_id);

-- for debugging --

-- INSERT INTO themes (author, epoch_open, theme_text) VALUES
-- ('user1', '2021-07-03 04:01:34.138663', 'theme1'),
-- ('user2', '2021-07-03 15:01:34.138663', '日本語でそれなりに長いお題 多分二行くらいにはなるかな？'),
-- ('user3', '2021-07-03 20:01:34.138663', '文字数の限界ああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああ'),
-- ('user4', '2021-07-04 04:01:34.138663', 'theme4'),
-- ('user5', '2021-07-04 15:01:34.138663', 'theme5'),
-- ('user6', '2021-07-04 20:01:34.138663', 'theme6');

-- INSERT INTO answers (user_id, theme_id, epoch_submit, answer_text, score, voted) VALUES
-- ('user1', 1, '2021-07-04 15:01:34.138663', 'user1_answer', 0, FALSE),
-- ('user2', 1, '2021-07-04 15:01:34.138663', 'user2の解答は本当にめちゃくちゃ長い！多分数行にまで亘る  というかそうなってほしい  めちゃ長いね', 0, FALSE),
-- ('user3', 1, '2021-07-04 15:01:34.138663', 'user3_answer', 0, TRUE),
-- ('user1', 2, '2021-07-04 15:01:34.138663', 'user1_answer', 0, FALSE),
-- ('user2', 2, '2021-07-04 15:02:34.138663', 'user2の解答は本当にめちゃくちゃ長い！多分数行にまで亘る  というかそうなってほしい  めちゃ長いね', 0, FALSE),
-- ('user3', 2, '2021-07-04 15:03:34.138663', 'user3_answer', 0, FALSE),
-- ('user1', 3, '2021-07-04 15:01:34.138663', 'user1_answer', 0, FALSE),
-- ('user2', 3, '2021-07-04 15:02:34.138663', 'user2の解答は本当にめちゃくちゃ長い！多分数行にまで亘る  というかそうなってほしい  めちゃ長いね', 0, FALSE),
-- ('user3', 3, '2021-07-04 15:03:34.138663', 'user3_answer', 0, FALSE),
-- ('user4', 5, '2021-07-04 21:01:34.138663', 'user4_answer', 0, FALSE),
-- ('user5', 5, '2021-07-04 21:05:34.138663', 'user5_answer', 0, FALSE);

-- INSERT INTO votes (user_id, theme_id, answer_id, score) VALUES
-- ('user3', 1, 1, 100000),
-- ('user3', 1, 2, 1),
-- ('user4', 1, 1, 100000),
-- ('user4', 1, 3, 1);