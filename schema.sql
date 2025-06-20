CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS authors(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name text UNIQUE NOT NULL,
    created_at timestamp NOT NULL DEFAULT now(),
    update_at timestamp NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS categories(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name character varying(64) UNIQUE NOT NULL,
    description text NOT NULL,
    created_at timestamp NOT NULL DEFAULT now(),
    update_at timestamp NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS books(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    isbn text UNIQUE NOT NULL,
    title text NOT NULL,
    description text NOT NULL,
    created_at timestamp NOT NULL DEFAULT now(),
    update_at timestamp NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS book_authors(
    book_id uuid NOT NULL references books(id) ON DELETE CASCADE,
    author_id uuid NOT NULL references authors(id) ON DELETE CASCADE,

    PRIMARY KEY (book_id, author_id)
);

CREATE TABLE IF NOT EXISTS book_categories(
    book_id uuid NOT NULL references books(id) ON DELETE CASCADE,
    category_id uuid NOT NULL references categories(id) ON DELETE CASCADE,

    PRIMARY KEY (book_id, category_id)
);

CREATE TYPE role AS ENUM('user', 'admin');

CREATE TABLE IF NOT EXISTS users(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    email character varying(128) NOT NULL UNIQUE,
    password text NOT NULL,
    role role NOT NULL DEFAULT 'user'::role,
    first_name character varying(32) NOT NULL,
    last_name character varying(32) NOT NULL,
    created_at timestamp NOT NULL DEFAULT now(),
    update_at timestamp NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS reviews(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    book_id uuid NOT NULL references books(id) ON DELETE CASCADE,
    user_id uuid NOT NULL references users(id) ON DELETE CASCADE,
    rate real NOT NULL,
    content text NOT NULL,
    created_at timestamp NOT NULL DEFAULT now(),
    update_at timestamp NOT NULL DEFAULT now()
);
