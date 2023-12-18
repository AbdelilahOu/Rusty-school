use ::migration::{Migrator, MigratorTrait};
use dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection as DbConn, DbErr};
use std::time::Duration;

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    dotenv::dotenv().ok();
    // get database url from env
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    // init connection options
    let mut options = ConnectOptions::new(db_url);
    options
        .max_connections(3)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(5));
    // get db connection
    let db_res = Database::connect(options).await;
    // check for errors
    match db_res {
        Ok(db_conn) => {
            println!("Database connection established.");
            Ok(db_conn)
        }
        Err(db_err) => Err(db_err),
    }
}

pub async fn run_migrations(db: &DbConn) -> Result<(), DbErr> {
    println!("Running migrations...");
    Migrator::up(db, None).await?;
    Ok(())
}
