use ::entity::student::Entity as Student;
use sea_orm::{prelude::Uuid, *};

pub struct Mutation;

impl Mutation {
    // pub async fn create_student(db: &DbConn, student: String) -> Result<(), DbErr> {
    //     Ok(())
    // }
    pub async fn delete_student(db: &DbConn, id: Uuid) -> Result<u64, String> {
        let student_res = Student::find_by_id(id).one(db).await;
        match student_res {
            Ok(student_opt) => match student_opt {
                Some(student_a) => match student_a.delete(db).await {
                    Ok(delete_a) => Ok(delete_a.rows_affected),
                    Err(db_err) => Err(db_err.to_string()),
                },
                None => Err(String::from("student doesnt exist")),
            },
            Err(db_err) => Err(db_err.to_string()),
        }
    }
    // pub async fn update_student(db: &DbConn) -> Result<(), DbErr> {
    //     Ok(())
    // }
}
