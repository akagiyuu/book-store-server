// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct InsertParams<T1: crate::StringSql> {
    pub book_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub rate: f32,
    pub content: T1,
}
#[derive(Debug)]
pub struct UpdateParams<T1: crate::StringSql> {
    pub rate: Option<f32>,
    pub content: Option<T1>,
    pub book_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
}
#[derive(Clone, Copy, Debug)]
pub struct DeleteParams {
    pub book_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetByBookId {
    pub book_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub rate: f32,
    pub content: String,
    pub created_at: crate::types::time::Timestamp,
    pub update_at: crate::types::time::Timestamp,
}
pub struct GetByBookIdBorrowed<'a> {
    pub book_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub rate: f32,
    pub content: &'a str,
    pub created_at: crate::types::time::Timestamp,
    pub update_at: crate::types::time::Timestamp,
}
impl<'a> From<GetByBookIdBorrowed<'a>> for GetByBookId {
    fn from(
        GetByBookIdBorrowed {
            book_id,
            user_id,
            rate,
            content,
            created_at,
            update_at,
        }: GetByBookIdBorrowed<'a>,
    ) -> Self {
        Self {
            book_id,
            user_id,
            rate,
            content: content.into(),
            created_at,
            update_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetByUserId {
    pub book_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub rate: f32,
    pub content: String,
    pub created_at: crate::types::time::Timestamp,
    pub update_at: crate::types::time::Timestamp,
}
pub struct GetByUserIdBorrowed<'a> {
    pub book_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub rate: f32,
    pub content: &'a str,
    pub created_at: crate::types::time::Timestamp,
    pub update_at: crate::types::time::Timestamp,
}
impl<'a> From<GetByUserIdBorrowed<'a>> for GetByUserId {
    fn from(
        GetByUserIdBorrowed {
            book_id,
            user_id,
            rate,
            content,
            created_at,
            update_at,
        }: GetByUserIdBorrowed<'a>,
    ) -> Self {
        Self {
            book_id,
            user_id,
            rate,
            content: content.into(),
            created_at,
            update_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetAll {
    pub book_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub rate: f32,
    pub content: String,
    pub created_at: crate::types::time::Timestamp,
    pub update_at: crate::types::time::Timestamp,
}
pub struct GetAllBorrowed<'a> {
    pub book_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub rate: f32,
    pub content: &'a str,
    pub created_at: crate::types::time::Timestamp,
    pub update_at: crate::types::time::Timestamp,
}
impl<'a> From<GetAllBorrowed<'a>> for GetAll {
    fn from(
        GetAllBorrowed {
            book_id,
            user_id,
            rate,
            content,
            created_at,
            update_at,
        }: GetAllBorrowed<'a>,
    ) -> Self {
        Self {
            book_id,
            user_id,
            rate,
            content: content.into(),
            created_at,
            update_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct GetByBookIdQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<GetByBookIdBorrowed, tokio_postgres::Error>,
    mapper: fn(GetByBookIdBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetByBookIdQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetByBookIdBorrowed) -> R,
    ) -> GetByBookIdQuery<'c, 'a, 's, C, R, N> {
        GetByBookIdQuery {
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
pub struct GetByUserIdQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<GetByUserIdBorrowed, tokio_postgres::Error>,
    mapper: fn(GetByUserIdBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetByUserIdQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetByUserIdBorrowed) -> R,
    ) -> GetByUserIdQuery<'c, 'a, 's, C, R, N> {
        GetByUserIdQuery {
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
        "INSERT INTO reviews(book_id, user_id, rate, content) VALUES ($1, $2, $3, $4)",
    ))
}
pub struct InsertStmt(crate::client::async_::Stmt);
impl InsertStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        book_id: &'a uuid::Uuid,
        user_id: &'a uuid::Uuid,
        rate: &'a f32,
        content: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(stmt, &[book_id, user_id, rate, content])
            .await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        InsertParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for InsertStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.book_id,
            &params.user_id,
            &params.rate,
            &params.content,
        ))
    }
}
pub fn get_by_book_id() -> GetByBookIdStmt {
    GetByBookIdStmt(crate::client::async_::Stmt::new(
        "SELECT * FROM reviews WHERE book_id = $1",
    ))
}
pub struct GetByBookIdStmt(crate::client::async_::Stmt);
impl GetByBookIdStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        book_id: &'a uuid::Uuid,
    ) -> GetByBookIdQuery<'c, 'a, 's, C, GetByBookId, 1> {
        GetByBookIdQuery {
            client,
            params: [book_id],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetByBookIdBorrowed, tokio_postgres::Error> {
                    Ok(GetByBookIdBorrowed {
                        book_id: row.try_get(0)?,
                        user_id: row.try_get(1)?,
                        rate: row.try_get(2)?,
                        content: row.try_get(3)?,
                        created_at: row.try_get(4)?,
                        update_at: row.try_get(5)?,
                    })
                },
            mapper: |it| GetByBookId::from(it),
        }
    }
}
pub fn get_by_user_id() -> GetByUserIdStmt {
    GetByUserIdStmt(crate::client::async_::Stmt::new(
        "SELECT * FROM reviews WHERE user_id = $1",
    ))
}
pub struct GetByUserIdStmt(crate::client::async_::Stmt);
impl GetByUserIdStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        user_id: &'a uuid::Uuid,
    ) -> GetByUserIdQuery<'c, 'a, 's, C, GetByUserId, 1> {
        GetByUserIdQuery {
            client,
            params: [user_id],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetByUserIdBorrowed, tokio_postgres::Error> {
                    Ok(GetByUserIdBorrowed {
                        book_id: row.try_get(0)?,
                        user_id: row.try_get(1)?,
                        rate: row.try_get(2)?,
                        content: row.try_get(3)?,
                        created_at: row.try_get(4)?,
                        update_at: row.try_get(5)?,
                    })
                },
            mapper: |it| GetByUserId::from(it),
        }
    }
}
pub fn get_all() -> GetAllStmt {
    GetAllStmt(crate::client::async_::Stmt::new("SELECT * FROM reviews"))
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
                        book_id: row.try_get(0)?,
                        user_id: row.try_get(1)?,
                        rate: row.try_get(2)?,
                        content: row.try_get(3)?,
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
        "UPDATE reviews SET rate = COALESCE(rate, $1), content = COALESCE(content, $2), update_at = now() WHERE book_id = $3 AND user_id = $4",
    ))
}
pub struct UpdateStmt(crate::client::async_::Stmt);
impl UpdateStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        rate: &'a Option<f32>,
        content: &'a Option<T1>,
        book_id: &'a uuid::Uuid,
        user_id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(stmt, &[rate, content, book_id, user_id])
            .await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.rate,
            &params.content,
            &params.book_id,
            &params.user_id,
        ))
    }
}
pub fn delete() -> DeleteStmt {
    DeleteStmt(crate::client::async_::Stmt::new(
        "DELETE FROM reviews WHERE book_id = $1 AND user_id = $2",
    ))
}
pub struct DeleteStmt(crate::client::async_::Stmt);
impl DeleteStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        book_id: &'a uuid::Uuid,
        user_id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[book_id, user_id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        DeleteParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeleteStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a DeleteParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.book_id, &params.user_id))
    }
}
