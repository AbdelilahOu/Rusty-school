use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection as DbConn, DbErr};

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let db_res = Database::connect(db_url).await;
    match db_res {
        Ok(db_conn) => {
            println!("Database connection established.");
            Ok(db_conn)
        }
        Err(db_err) => Err(db_err),
    }
}
