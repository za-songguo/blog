-- 用于存储文章数据

CREATE TABLE
    articles(
        id SERIAL NOT NULL,
        title character varying(255) NOT NULL,
        content text NOT NULL,
        date date NOT NULL DEFAULT CURRENT_DATE,
        PRIMARY KEY(id)
    );

-- 用于存储文章的评论数据

CREATE TABLE
    comments(
        id SERIAL NOT NULL,
        user_id integer NOT NULL,
        content character varying(1024) NOT NULL,
        date date NOT NULL DEFAULT CURRENT_DATE,
        article integer NOT NULL,
        PRIMARY KEY(id)
    );

-- 用于存储使用 Github 登录过的用户信息

CREATE TABLE
    users(
        id integer NOT NULL,
        name character varying(255) NOT NULL,
        avatar_url character varying(255) NOT NULL,
        PRIMARY KEY(id)
    );