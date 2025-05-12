--! insert
INSERT INTO users(email, password, first_name, last_name)
VALUES (:email, :password, :first_name, :last_name);

--! get
SELECT * FROM users WHERE id = :id;

--! get_all
SELECT * FROM users;

--! update (email?, password?, first_name?, last_name?)
UPDATE users
SET
    email = COALESCE(email, :email),
    password = COALESCE(password, :password),
    first_name = COALESCE(first_name, :first_name),
    last_name = COALESCE(last_name, :last_name),
    update_at = now()
WHERE id = :id;

--! delete
DELETE FROM users WHERE id = :id;
