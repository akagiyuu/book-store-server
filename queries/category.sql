--! insert
INSERT INTO categories(name, description)
VALUES (:name, :description);

--! get
SELECT * FROM categories WHERE id = :id;

--! get_all
SELECT * FROM categories;

--! update (name?, description?)
UPDATE categories
SET
    name = COALESCE(name, :name),
    description = COALESCE(description, :description),
    update_at = now()
WHERE id = :id;

--! delete
DELETE FROM categories WHERE id = :id;
