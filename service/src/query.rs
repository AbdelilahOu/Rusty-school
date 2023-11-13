use crate::{GStudent, ListQuery};
use ::entity::students::Entity as Student;
use sea_orm::{prelude::Uuid, *};

pub struct ServiceQuery;

impl ServiceQuery {
    pub async fn list_students(queries: ListQuery, db: &DbConn) -> Result<Vec<GStudent>, String> {
        let list_students = Student::find()
            .offset((queries.page - 1) * queries.limit)
            .limit(queries.limit)
            .all(db)
            .await;
        match list_students {
            Ok(students) => {
                let maped_students = students.into_iter().map(|student| GStudent {
                    id: student.id,
                    first_name: student.first_name,
                    last_name: student.last_name,
                    address: student.address,
                    level: student.level,
                });
                Ok(maped_students.collect())
            }
            Err(err) => Err(err.to_string()),
        }
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
