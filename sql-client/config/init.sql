DROP TABLE IF EXISTS users;
CREATE TABLE users (
  user_id       VARCHAR(30) NOT NULL,
  display_name  VARCHAR(70),
  hash          VARCHAR(128) NOT NULL,
  login_session VARCHAR(128) NOT NULL,
  PRIMARY KEY (user_id)
);
CREATE INDEX ON users (LOWER(user_id));

DROP TABLE IF EXISTS themes;
CREATE TABLE themes (
  id            SERIAL NOT NULL,
  user_id       VARCHAR(30) NOT NULL,
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
  user_id       VARCHAR(30) NOT NULL,
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
  user_id       VARCHAR(30) NOT NULL,
  epoch_login   TIMESTAMPTZ NOT NULL,
  PRIMARY KEY (user_id)
);

DROP TABLE IF EXISTS votes;
CREATE TABLE votes (
  user_id      VARCHAR(30) NOT NULL,
  theme_id     INTEGER NOT NULL,
  answer_id    INTEGER NOT NULL,
  score        INTEGER NOT NULL,
  PRIMARY KEY (user_id, theme_id, answer_id)
);
CREATE INDEX ON votes (user_id, theme_id);
CREATE INDEX ON votes (theme_id);

DROP TABLE IF EXISTS profiles;
CREATE TABLE profiles (
  user_id      VARCHAR(30) NOT NULL,
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
('$user1', 'USER 1', '', ''),
('$user2', NULL, '', ''),
('$user3', 'USER 3', '', ''),
('$user4', 'USER 4', '', '');

INSERT INTO themes (user_id, epoch_open, theme_text) VALUES
('admin', '2021-07-11 04:00:00.000000+09:00', 'theme1'),
('admin', '2021-07-11 12:00:00.000000+09:00', 'theme2'),
('admin', '2021-07-11 20:00:00.000000+09:00', 'theme3'),
('admin', '2021-07-12 04:00:00.000000+09:00', '日本一安い旅館、どんなの？'),
('admin', '2021-07-12 12:00:00.000000+09:00', '「この忍者、センスないな」どうしてそう思った？'),
('admin', '2021-07-12 20:00:00.000000+09:00', 'ランチ5000円のサイゼリヤ、普通と何が違う？'),
('admin', '2021-07-13 04:00:00.000000+09:00', '「あ、このお坊さん偽物だな」なぜ分かった？'),
('admin', '2021-07-13 12:00:00.000000+09:00', '引っ越しの達人の特徴を教えてください'),
('admin', '2021-07-13 20:00:00.000000+09:00', '卒業式のしおりに書かれていた驚きのプログラムとは？');

INSERT INTO answers (user_id, theme_id, epoch_submit, answer_text, score, voted) VALUES
('$user1', 4, '2021-07-09 15:01:34.138663+09:00', 'user1_answer', 0, FALSE),
('$user2', 4, '2021-07-09 15:01:34.138663+09:00', 'user2_answer', 0, FALSE),
('$user3', 4, '2021-07-09 15:01:34.138663+09:00', 'user3_answer', 0, TRUE),
('$user1', 5, '2021-07-09 15:01:34.138663+09:00', 'user1_answer', 0, FALSE),
('$user2', 5, '2021-07-09 15:02:34.138663+09:00', 'user2_answer', 0, FALSE),
('$user3', 5, '2021-07-09 15:03:34.138663+09:00', 'user3_answer', 0, FALSE),
('$user1', 6, '2021-07-09 15:01:34.138663+09:00', 'user1_answer', 0, FALSE),
('$user2', 6, '2021-07-09 15:02:34.138663+09:00', 'user2_answer', 0, FALSE),
('$user3', 6, '2021-07-09 15:03:34.138663+09:00', 'user3_answer', 0, FALSE);

INSERT INTO votes (user_id, theme_id, answer_id, score) VALUES
('$user3', 4, 1, 100000),
('$user3', 4, 2, 1),
('$user2', 4, 1, 100000),
('$user2', 4, 3, 1),
('$user3', 5, 4, 100000),
('$user3', 5, 5, 1),
('$user2', 5, 4, 100000),
('$user2', 5, 6, 1),
('$user3', 6, 7, 100000),
('$user3', 6, 8, 1),
('$user2', 6, 7, 100000),
('$user2', 6, 9, 1);