--! insert
INSERT INTO reviews(book_id, user_id, rate, content)
VALUES (:book_id, :user_id, :rate, :content);

--! get_by_book_id
SELECT * FROM reviews WHERE book_id = :book_id;

--! get_by_user_id
SELECT * FROM reviews WHERE user_id = :user_id;

--! get_all
SELECT * FROM reviews;

--! update (rate?, content?)
UPDATE reviews
SET
    rate = COALESCE(rate, :rate),
    content = COALESCE(content, :content),
    update_at = now()
WHERE book_id = :book_id AND user_id = :user_id;

--! delete
DELETE FROM reviews WHERE book_id = :book_id AND user_id = :user_id;
