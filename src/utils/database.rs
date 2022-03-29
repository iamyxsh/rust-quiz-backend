use tokio_postgres::{connect, Client, NoTls};

pub const USER_TABLE_NAME: &str = "users";

pub async fn db_init() -> (u64, String) {
    create_user_table().await
}

async fn create_user_table() -> (u64, String) {
    let columns = vec![
        "id         SERIAL PRIMARY KEY".to_string(),
        "name       VARCHAR NOT NULL".to_string(),
        "email      VARCHAR NOT NULL".to_string(),
        "password   TEXT NOT NULL".to_string(),
    ];
    create_table(USER_TABLE_NAME.to_string(), columns).await
}

pub async fn return_client() -> Client {
    let (client, connection) = connect(&dotenv::var("PG_STRING").unwrap(), NoTls)
        .await
        .unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}

async fn create_table(table_name: String, columns: Vec<String>) -> (u64, String) {
    let client = return_client().await;
    let tx = client
        .execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} ( {} )",
                table_name,
                columns.join(",")
            ),
            &[],
        )
        .await;

    match tx {
        Ok(tx) => (tx, "".to_string()),
        Err(err) => (0, err.to_string()),
    }
}
