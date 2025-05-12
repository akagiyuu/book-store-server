// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct InsertParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub isbn: T1,
    pub title: T2,
    pub description: T3,
}
#[derive(Clone, Copy, Debug)]
pub struct InsertAuthorParams {
    pub id: uuid::Uuid,
    pub author_id: uuid::Uuid,
}
#[derive(Clone, Copy, Debug)]
pub struct InsertCategoryParams {
    pub id: uuid::Uuid,
    pub category_id: uuid::Uuid,
}
#[derive(Debug)]
pub struct UpdateParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub isbn: Option<T1>,
    pub title: Option<T2>,
    pub description: Option<T3>,
    pub id: uuid::Uuid,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Get {
    pub id: uuid::Uuid,
    pub isbn: String,
    pub title: String,
    pub description: String,
    pub created_at: crate::types::time::Timestamp,
    pub update_at: crate::types::time::Timestamp,
}
pub struct GetBorrowed<'a> {
    pub id: uuid::Uuid,
    pub isbn: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub created_at: crate::types::time::Timestamp,
    pub update_at: crate::types::time::Timestamp,
}
impl<'a> From<GetBorrowed<'a>> for Get {
    fn from(
        GetBorrowed {
            id,
            isbn,
            title,
            description,
            created_at,
            update_at,
        }: GetBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            isbn: isbn.into(),
            title: title.into(),
            description: description.into(),
            created_at,
            update_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetAll {
    pub id: uuid::Uuid,
    pub isbn: String,
    pub title: String,
    pub description: String,
    pub created_at: crate::types::time::Timestamp,
    pub update_at: crate::types::time::Timestamp,
}
pub struct GetAllBorrowed<'a> {
    pub id: uuid::Uuid,
    pub isbn: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub created_at: crate::types::time::Timestamp,
    pub update_at: crate::types::time::Timestamp,
}
impl<'a> From<GetAllBorrowed<'a>> for GetAll {
    fn from(
        GetAllBorrowed {
            id,
            isbn,
            title,
            description,
            created_at,
            update_at,
        }: GetAllBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            isbn: isbn.into(),
            title: title.into(),
            description: description.into(),
            created_at,
            update_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct GetQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<GetBorrowed, tokio_postgres::Error>,
    mapper: fn(GetBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(GetBorrowed) -> R) -> GetQuery<'c, 'a, 's, C, R, N> {
        GetQuery {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(it)
    }
}
pub struct StringQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<&str, tokio_postgres::Error>,
    mapper: fn(&str) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> StringQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'c, 'a, 's, C, R, N> {
        StringQuery {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(it)
    }
}
pub struct GetAllQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<GetAllBorrowed, tokio_postgres::Error>,
    mapper: fn(GetAllBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetAllQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(GetAllBorrowed) -> R) -> GetAllQuery<'c, 'a, 's, C, R, N> {
        GetAllQuery {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(it)
    }
}
pub fn insert() -> InsertStmt {
    InsertStmt(crate::client::async_::Stmt::new(
        "INSERT INTO books(isbn, title, description) VALUES ($1, $2, $3)",
    ))
}
pub struct InsertStmt(crate::client::async_::Stmt);
impl InsertStmt {
    pub async fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'s mut self,
        client: &'c C,
        isbn: &'a T1,
        title: &'a T2,
        description: &'a T3,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[isbn, title, description]).await
    }
}
impl<
    'a,
    C: GenericClient + Send + Sync,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        InsertParams<T1, T2, T3>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for InsertStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertParams<T1, T2, T3>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.isbn, &params.title, &params.description))
    }
}
pub fn insert_author() -> InsertAuthorStmt {
    InsertAuthorStmt(crate::client::async_::Stmt::new(
        "INSERT INTO book_authors(book_id, author_id) VALUES ($1, $2)",
    ))
}
pub struct InsertAuthorStmt(crate::client::async_::Stmt);
impl InsertAuthorStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        id: &'a uuid::Uuid,
        author_id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[id, author_id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        InsertAuthorParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for InsertAuthorStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertAuthorParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.id, &params.author_id))
    }
}
pub fn insert_category() -> InsertCategoryStmt {
    InsertCategoryStmt(crate::client::async_::Stmt::new(
        "INSERT INTO book_categories(book_id, category_id) VALUES ($1, $2)",
    ))
}
pub struct InsertCategoryStmt(crate::client::async_::Stmt);
impl InsertCategoryStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        id: &'a uuid::Uuid,
        category_id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[id, category_id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        InsertCategoryParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for InsertCategoryStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertCategoryParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.id, &params.category_id))
    }
}
pub fn get() -> GetStmt {
    GetStmt(crate::client::async_::Stmt::new(
        "SELECT * FROM books WHERE id = $1",
    ))
}
pub struct GetStmt(crate::client::async_::Stmt);
impl GetStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        id: &'a uuid::Uuid,
    ) -> GetQuery<'c, 'a, 's, C, Get, 1> {
        GetQuery {
            client,
            params: [id],
            stmt: &mut self.0,
            extractor: |row: &tokio_postgres::Row| -> Result<GetBorrowed, tokio_postgres::Error> {
                Ok(GetBorrowed {
                    id: row.try_get(0)?,
                    isbn: row.try_get(1)?,
                    title: row.try_get(2)?,
                    description: row.try_get(3)?,
                    created_at: row.try_get(4)?,
                    update_at: row.try_get(5)?,
                })
            },
            mapper: |it| Get::from(it),
        }
    }
}
pub fn get_author() -> GetAuthorStmt {
    GetAuthorStmt(crate::client::async_::Stmt::new(
        "SELECT name FROM authors WHERE id = ( SELECT author_id FROM book_authors WHERE book_id = $1 )",
    ))
}
pub struct GetAuthorStmt(crate::client::async_::Stmt);
impl GetAuthorStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        id: &'a uuid::Uuid,
    ) -> StringQuery<'c, 'a, 's, C, String, 1> {
        StringQuery {
            client,
            params: [id],
            stmt: &mut self.0,
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub fn get_category() -> GetCategoryStmt {
    GetCategoryStmt(crate::client::async_::Stmt::new(
        "SELECT name FROM authors WHERE id = ( SELECT author_id FROM book_authors WHERE book_id = $1 )",
    ))
}
pub struct GetCategoryStmt(crate::client::async_::Stmt);
impl GetCategoryStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        id: &'a uuid::Uuid,
    ) -> StringQuery<'c, 'a, 's, C, String, 1> {
        StringQuery {
            client,
            params: [id],
            stmt: &mut self.0,
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub fn get_all() -> GetAllStmt {
    GetAllStmt(crate::client::async_::Stmt::new("SELECT * FROM books"))
}
pub struct GetAllStmt(crate::client::async_::Stmt);
impl GetAllStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> GetAllQuery<'c, 'a, 's, C, GetAll, 0> {
        GetAllQuery {
            client,
            params: [],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetAllBorrowed, tokio_postgres::Error> {
                    Ok(GetAllBorrowed {
                        id: row.try_get(0)?,
                        isbn: row.try_get(1)?,
                        title: row.try_get(2)?,
                        description: row.try_get(3)?,
                        created_at: row.try_get(4)?,
                        update_at: row.try_get(5)?,
                    })
                },
            mapper: |it| GetAll::from(it),
        }
    }
}
pub fn update() -> UpdateStmt {
    UpdateStmt(crate::client::async_::Stmt::new(
        "UPDATE books SET isbn = COALESCE(isbn, $1), title = COALESCE(title, $2), description = COALESCE(description, $3), update_at = now() WHERE id = $4",
    ))
}
pub struct UpdateStmt(crate::client::async_::Stmt);
impl UpdateStmt {
    pub async fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'s mut self,
        client: &'c C,
        isbn: &'a Option<T1>,
        title: &'a Option<T2>,
        description: &'a Option<T3>,
        id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[isbn, title, description, id]).await
    }
}
impl<
    'a,
    C: GenericClient + Send + Sync,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateParams<T1, T2, T3>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateParams<T1, T2, T3>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.isbn,
            &params.title,
            &params.description,
            &params.id,
        ))
    }
}
pub fn delete() -> DeleteStmt {
    DeleteStmt(crate::client::async_::Stmt::new(
        "DELETE FROM books WHERE id = $1",
    ))
}
pub struct DeleteStmt(crate::client::async_::Stmt);
impl DeleteStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[id]).await
    }
}
pub fn delete_author() -> DeleteAuthorStmt {
    DeleteAuthorStmt(crate::client::async_::Stmt::new(
        "DELETE FROM book_authors WHERE book_id = $1",
    ))
}
pub struct DeleteAuthorStmt(crate::client::async_::Stmt);
impl DeleteAuthorStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[id]).await
    }
}
pub fn delete_category() -> DeleteCategoryStmt {
    DeleteCategoryStmt(crate::client::async_::Stmt::new(
        "DELETE FROM book_categories WHERE book_id = $1",
    ))
}
pub struct DeleteCategoryStmt(crate::client::async_::Stmt);
impl DeleteCategoryStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[id]).await
    }
}
