use ::entity::students::{ActiveModel, Entity as Student};
use sea_orm::{prelude::Uuid, *};

use crate::CStudent;

pub struct ServiceMutation;

impl ServiceMutation {
    pub async fn create_student(db: &DbConn, data: CStudent) -> Result<Uuid, DbErr> {
        let c_student = ActiveModel {
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            address: Set(data.address),
            level: Set(data.level),
            ..Default::default()
        };
        match Student::insert(c_student).exec(db).await {
            Ok(res) => Ok(res.last_insert_id),
            Err(err) => return Err(err),
        }
    }
    pub async fn delete_student(db: &DbConn, id: Uuid) -> Result<u64, String> {
        let selected_student = Student::find_by_id(id).one(db).await;
        match selected_student {
            Ok(student_opt) => match student_opt {
                Some(student_model) => match student_model.delete(db).await {
                    Ok(delete_a) => Ok(delete_a.rows_affected),
                    Err(err) => Err(err.to_string()),
                },
                None => Err(String::from("student doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn update_student(db: &DbConn, id: Uuid, data: CStudent) -> Result<CStudent, String> {
        let selected_student = Student::find_by_id(id).one(db).await;
        match selected_student {
            Ok(student_opt) => match student_opt {
                Some(student_model) => {
                    let mut student_model: ActiveModel = student_model.into();
                    student_model.first_name = Set(data.first_name);
                    student_model.last_name = Set(data.last_name);
                    student_model.address = Set(data.address);
                    student_model.level = Set(data.level);
                    student_model.id = Set(id);
                    match student_model.update(db).await {
                        Ok(updated_student) => Ok(CStudent {
                            first_name: updated_student.first_name,
                            last_name: updated_student.last_name,
                            address: updated_student.address,
                            level: updated_student.level,
                        }),
                        Err(err) => Err(err.to_string()),
                    }
                }
                None => Err(String::from("student doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
}
