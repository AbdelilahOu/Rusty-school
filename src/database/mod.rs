use ::service::sea_orm::{ConnectOptions, Database, DatabaseConnection as DbConn, DbErr};
use std::time::Duration;

pub async fn establish_connection(db_url: String) -> Result<DbConn, DbErr> {
    // init connection options
    let mut options = ConnectOptions::new(db_url);
    options
        .max_connections(3)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(10));
    // get db connection
    let db_conn = Database::connect(options).await?;
    println!("Database connection established.");
    Ok(db_conn)
}
