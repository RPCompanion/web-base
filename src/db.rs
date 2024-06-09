
use std::sync::OnceLock;
use tokio;
use postgres::NoTls;
use tokio_postgres::{Client, Error};

static POSTGRES_CONNECTION: OnceLock<String> = OnceLock::new();

pub async fn get_async_connection(postgres_connection: Option<String>) -> Result<Client, Error> {

    let (client, connection) = match postgres_connection {

        Some(connection) => {
            tokio_postgres::connect(&connection, NoTls).await?
        },
        None => { 

            if let Some(connection) = POSTGRES_CONNECTION.get() {
                tokio_postgres::connect(&connection, NoTls).await?
            } else {
               tokio_postgres::connect("", NoTls).await?
            }

        }

    };

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {

        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
        
    });

     Ok(client)

}

pub fn set_global_connection(connection: String) -> Result<(), String> {
    POSTGRES_CONNECTION.set(connection)
}