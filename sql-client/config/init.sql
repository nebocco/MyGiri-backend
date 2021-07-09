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
  id            SERIAL NOT NULL,
  user_id       VARCHAR(100) NOT NULL,
  epoch_open    TIMESTAMPTZ NOT NULL,
  theme_text    VARCHAR(255) NOT NULL,
  updated       BOOLEAN NOT NULL DEFAULT FALSE,
  PRIMARY KEY (id)
);
CREATE INDEX ON themes (user_id);
CREATE INDEX ON themes (epoch_open);

DROP TABLE IF EXISTS answers;
CREATE TABLE answers (
  id            SERIAL,
  user_id       VARCHAR(100) NOT NULL,
  theme_id      INTEGER NOT NULL,
  epoch_submit  TIMESTAMPTZ NOT NULL,
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
  epoch_login   TIMESTAMPTZ NOT NULL,
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

DROP TABLE IF EXISTS profiles;
CREATE TABLE profiles (
  user_id      VARCHAR(100) NOT NULL,
  heart        INTEGER,
  star         INTEGER,
  answer       INTEGER,
  theme        INTEGER,
  self_vote    INTEGER,
  top_count    INTEGER,
  PRIMARY KEY (user_id)
);

-- for debugging --

INSERT INTO users (user_id, display_name, hash, login_session) VALUES
('user1', 'USER 1', '', ''),
('user2', NULL, '', ''),
('user3', 'USER 3', '', ''),
('user4', 'USER 4', '', '');

INSERT INTO themes (user_id, epoch_open, theme_text) VALUES
('user1', '2021-07-08 04:01:34.138663+09:00', 'theme1'),
('user2', '2021-07-08 12:01:34.138663+09:00', '日本語でそれなりに長いお題 多分二行くらいにはなるかな？'),
('user3', '2021-07-08 20:01:34.138663+09:00', '文字数の限界ああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああああ'),
('user4', '2021-07-09 04:01:34.138663+09:00', 'theme4'),
('user5', '2021-07-09 12:01:34.138663+09:00', 'theme5'),
('user6', '2021-07-09 20:01:34.138663+09:00', 'theme6');

INSERT INTO answers (user_id, theme_id, epoch_submit, answer_text, score, voted) VALUES
('user1', 1, '2021-07-06 15:01:34.138663+09:00', 'user1_answer', 0, FALSE),
('user2', 1, '2021-07-06 15:01:34.138663+09:00', 'user2の解答は本当にめちゃくちゃ長い！多分数行にまで亘る  というかそうなってほしい  めちゃ長いね', 0, FALSE),
('user3', 1, '2021-07-06 15:01:34.138663+09:00', 'user3_answer', 0, TRUE),
('user1', 2, '2021-07-06 15:01:34.138663+09:00', 'user1_answer', 0, FALSE),
('user2', 2, '2021-07-06 15:02:34.138663+09:00', 'user2の解答は本当にめちゃくちゃ長い！多分数行にまで亘る  というかそうなってほしい  めちゃ長いね', 0, FALSE),
('user3', 2, '2021-07-06 15:03:34.138663+09:00', 'user3_answer', 0, FALSE),
('user1', 3, '2021-07-06 15:01:34.138663+09:00', 'user1_answer', 0, FALSE),
('user2', 3, '2021-07-06 15:02:34.138663+09:00', 'user2の解答は本当にめちゃくちゃ長い！多分数行にまで亘る  というかそうなってほしい  めちゃ長いね', 0, FALSE),
('user3', 3, '2021-07-06 15:03:34.138663+09:00', 'user3_answer', 0, FALSE),
('user4', 5, '2021-07-06 21:01:34.138663+09:00', 'user4_answer', 0, FALSE),
('user5', 5, '2021-07-06 21:05:34.138663+09:00', 'user5_answer', 0, FALSE);

INSERT INTO votes (user_id, theme_id, answer_id, score) VALUES
('user3', 1, 1, 100000),
('user3', 1, 2, 1),
('user4', 1, 1, 100000),
('user4', 1, 3, 1);