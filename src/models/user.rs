use crate::utils::database::{self, USER_TABLE_NAME};
use postgres_types::FromSql;
use serde::{Deserialize, Serialize};
use tokio_postgres::{types::ToSql, Error, Row};

#[derive(Debug, Serialize, Deserialize, ToSql, FromSql)]
pub struct User {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub email: String,
    pub password: String,
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        Self {
            id: Some(row.get(0)),
            name: row.get(1),
            email: row.get(2),
            password: row.get(3),
        }
    }
}

impl User {
    pub async fn insert_user(&self) -> Result<u64, Error> {
        let client = database::return_client().await;
        client
            .execute(
                &format!(
                    "INSERT INTO {} (name, email, password) VALUES ($1, $2, $3)",
                    USER_TABLE_NAME.to_string()
                )
                .to_string(),
                &[&self.name, &self.email, &self.password],
            )
            .await
    }

    pub async fn get_user(&mut self, col: String, val: String) -> Result<(), Error> {
        let client = database::return_client().await;
        let row = client
            .query(
                &format!(
                    "SELECT id, name, email, password FROM {} WHERE {} = '{}'",
                    USER_TABLE_NAME, col, val
                ),
                &[],
            )
            .await;

        match row {
            Ok(r) => {
                if r.len() > 0 {
                    self.id = Some(r[0].get(0));
                    self.name = r[0].get(1);
                    self.email = r[0].get(2);
                    self.password = r[0].get(3);
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
