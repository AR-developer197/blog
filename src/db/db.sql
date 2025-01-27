CREATE TABLE users (
    user_id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    username varchar(50) NOT NULL UNIQUE,
    password varchar(100) NOT NULL
);

CREATE TABLE posts (
    post_id varchar(100) GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id int REFERENCES users(user_id),
    title text NOT NULL,
    body text NOT NULL,
    publication_date DATE NOT NULL
);