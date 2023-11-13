use crate::GStudent;
use ::entity::students::Entity as Student;
use sea_orm::{prelude::Uuid, *};

pub struct Query;

impl Query {
    pub async fn list_students() -> Result<(), DbErr> {
        Ok(())
    }
    pub async fn get_student(id: Uuid, db: &DbConn) -> Result<GStudent, String> {
        let selected_student = Student::find_by_id(id).one(db).await;
        match selected_student {
            Ok(student_op) => match student_op {
                Some(some_student) => Ok(GStudent {
                    id: some_student.id,
                    first_name: some_student.first_name,
                    last_name: some_student.last_name,
                    address: some_student.address,
                    level: some_student.level,
                }),
                None => Err(String::from("student doesnt exist")),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}
