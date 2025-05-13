// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct InsertParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
> {
    pub email: T1,
    pub password: T2,
    pub first_name: T3,
    pub last_name: T4,
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetByEmail {
    pub id: uuid::Uuid,
    pub password: String,
}
pub struct GetByEmailBorrowed<'a> {
    pub id: uuid::Uuid,
    pub password: &'a str,
}
impl<'a> From<GetByEmailBorrowed<'a>> for GetByEmail {
    fn from(GetByEmailBorrowed { id, password }: GetByEmailBorrowed<'a>) -> Self {
        Self {
            id,
            password: password.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct GetByEmailQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<GetByEmailBorrowed, tokio_postgres::Error>,
    mapper: fn(GetByEmailBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetByEmailQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetByEmailBorrowed) -> R,
    ) -> GetByEmailQuery<'c, 'a, 's, C, R, N> {
        GetByEmailQuery {
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
        "INSERT INTO users(email, password, first_name, last_name) VALUES ($1, $2, $3, $4)",
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
        T4: crate::StringSql,
    >(
        &'s mut self,
        client: &'c C,
        email: &'a T1,
        password: &'a T2,
        first_name: &'a T3,
        last_name: &'a T4,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(stmt, &[email, password, first_name, last_name])
            .await
    }
}
impl<
    'a,
    C: GenericClient + Send + Sync,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        InsertParams<T1, T2, T3, T4>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for InsertStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertParams<T1, T2, T3, T4>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.email,
            &params.password,
            &params.first_name,
            &params.last_name,
        ))
    }
}
pub fn get_by_email() -> GetByEmailStmt {
    GetByEmailStmt(crate::client::async_::Stmt::new(
        "SELECT id, password FROM users WHERE email = $1 LIMIT 1",
    ))
}
pub struct GetByEmailStmt(crate::client::async_::Stmt);
impl GetByEmailStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        email: &'a T1,
    ) -> GetByEmailQuery<'c, 'a, 's, C, GetByEmail, 1> {
        GetByEmailQuery {
            client,
            params: [email],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetByEmailBorrowed, tokio_postgres::Error> {
                    Ok(GetByEmailBorrowed {
                        id: row.try_get(0)?,
                        password: row.try_get(1)?,
                    })
                },
            mapper: |it| GetByEmail::from(it),
        }
    }
}
