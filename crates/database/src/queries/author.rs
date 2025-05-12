// This file was generated with `clorinde`. Do not modify.

use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub fn insert() -> InsertStmt {
    InsertStmt(crate::client::async_::Stmt::new(
        "INSERT INTO authors(name) VALUES ($1)",
    ))
}
pub struct InsertStmt(crate::client::async_::Stmt);
impl InsertStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        name: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[name]).await
    }
}
