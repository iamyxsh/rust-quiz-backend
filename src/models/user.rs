use crate::utils::database::{self, USER_TABLE_NAME};
use serde::{Deserialize, Serialize};
use tokio_postgres::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub email: String,
    pub password: String,
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
        println!("{} {}", col, val);
        let client = database::return_client().await;
        let row = client
            .query(
                &format!("SELECT id, name, email, password FROM {}", USER_TABLE_NAME),
                &[],
            )
            .await;

        match row {
            Ok(r) => {
                println!("{:#?}", r);
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
