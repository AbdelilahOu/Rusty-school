// use sea_orm::{DbErr, RuntimeErr};

// pub async fn handle_error(error: DbErr) {
//     match error {
//         DbErr::Exec(RuntimeErr::SqlxError(error)) => match error {
//             sqlx::Error::Database(e) => {
//                 println!("Database error: {}", e);
//                 // We check the error code thrown by the database (MySQL in this case),
//                 // `23000` means `ER_DUP_KEY`: we have a duplicate key in the table.
//                 assert_eq!(e.code().unwrap(), "23000");
//             }
//             _ => println!("Unexpected sqlx::Error kind"),
//         },
//         _ => println!("Unexpected DbErr kind"),
//     }
// }
