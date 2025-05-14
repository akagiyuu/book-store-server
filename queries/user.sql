--! insert
INSERT INTO users(email, password, first_name, last_name)
VALUES (:email, :password, :first_name, :last_name);

--! get_by_email
SELECT id, password FROM users WHERE email = :email LIMIT 1;
