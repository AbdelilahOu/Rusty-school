use migration::{Migrator, MigratorTrait};
use service::sea_orm::{ConnectOptions, Database, DatabaseConnection as DbConn, DbErr};
use std::time::Duration;

pub async fn establish_connection(db_url: String) -> Result<DbConn, DbErr> {
    // init connection options
    let mut options = ConnectOptions::new(db_url);
    options
        .max_connections(3)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(10));
    // get db connection
    let db_res = Database::connect(options).await;
    // check for errors
    match db_res {
        Ok(db_conn) => {
            println!("Database connection established.");
            Ok(db_conn)
        }
        Err(db_err) => {
            println!("Database connection error: {:?}", db_err.to_string());
            Err(db_err)
        }
    }
}

pub async fn run_migrations(db: &DbConn) -> Result<(), DbErr> {
    println!("Running migrations...");
    Migrator::up(db, None).await?;
    Ok(())
}
