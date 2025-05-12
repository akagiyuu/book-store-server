--! insert
INSERT INTO authors(name)
VALUES (:name);

--! get
SELECT * FROM authors WHERE id = :id;

--! get_all
SELECT * FROM authors;

--! update (name?)
UPDATE authors
SET
    name = COALESCE(name, :name),
    update_at = now()
WHERE id = :id;

--! delete
DELETE FROM authors WHERE id = :id;
