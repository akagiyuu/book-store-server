--! insert
INSERT INTO books(isbn, title, description)
VALUES (:isbn, :title, :description);
--! insert_author
INSERT INTO book_authors(book_id, author_id)
VALUES (:id, :author_id);
--! insert_category
INSERT INTO book_categories(book_id, category_id)
VALUES (:id, :category_id);

--! get
SELECT * FROM books WHERE id = :id;
--! get_author
SELECT name
FROM authors
WHERE id = (
    SELECT author_id
    FROM book_authors
    WHERE book_id = :id
);
--! get_category
SELECT name
FROM authors
WHERE id = (
    SELECT author_id
    FROM book_authors
    WHERE book_id = :id
);

--! get_all
SELECT * FROM books;

--! update (isbn?, title?, description?)
UPDATE books
SET
    isbn = COALESCE(isbn, :isbn),
    title = COALESCE(title, :title),
    description = COALESCE(description, :description),
    update_at = now()
WHERE id = :id;

--! delete
DELETE FROM books WHERE id = :id;
--! delete_author
DELETE FROM book_authors WHERE book_id = :id;
--! delete_category
DELETE FROM book_categories WHERE book_id = :id;
