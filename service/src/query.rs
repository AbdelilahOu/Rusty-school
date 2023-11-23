use super::types::*;
use super::utils::filters::*;
use ::entity::contacts_informations::Entity as Contact;
use ::entity::parents::Entity as Parent;
use ::entity::students::Entity as Student;
use ::entity::teachers::Entity as Teacher;
use sea_orm::{prelude::Uuid, *};

pub struct ServiceQuery;

impl ServiceQuery {
    // students entity
    pub async fn list_students(qf: QueriesFilters, db: &DbConn) -> Result<Vec<GStudent>, String> {
        let list_students = Student::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .filter(generate_student_filters(qf.filters))
            .all(db)
            .await;

        match list_students {
            Ok(students) => {
                let maped_students = students.into_iter().map(|student| GStudent {
                    id: student.id,
                    first_name: student.first_name,
                    last_name: student.last_name,
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
                    level: some_student.level,
                }),
                None => Err(String::from("student doesnt exist")),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    //
    pub async fn list_teachers(qf: QueriesFilters, db: &DbConn) -> Result<Vec<GTeacher>, String> {
        let list_teachers = Teacher::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .filter(generate_teacher_filters(qf.filters))
            .all(db)
            .await;

        match list_teachers {
            Ok(teachers) => {
                let maped_teachers = teachers.into_iter().map(|teacher| GTeacher {
                    id: teacher.id,
                    first_name: teacher.first_name,
                    last_name: teacher.last_name,
                });
                Ok(maped_teachers.collect())
            }
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn get_teacher(id: Uuid, db: &DbConn) -> Result<GTeacher, String> {
        let selected_teacher = Teacher::find_by_id(id).one(db).await;
        match selected_teacher {
            Ok(teacher_op) => match teacher_op {
                Some(some_teacher) => Ok(GTeacher {
                    id: some_teacher.id,
                    first_name: some_teacher.first_name,
                    last_name: some_teacher.last_name,
                }),
                None => Err(String::from("teacher doesnt exist")),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    //
    pub async fn list_parents(qf: QueriesFilters, db: &DbConn) -> Result<Vec<GParent>, String> {
        let list_parents = Parent::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .filter(generate_parent_filters(qf.filters))
            .all(db)
            .await;

        match list_parents {
            Ok(parents) => {
                let maped_parents = parents.into_iter().map(|parent| GParent {
                    id: parent.id,
                    first_name: parent.first_name,
                    last_name: parent.last_name,
                });
                Ok(maped_parents.collect())
            }
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn get_parent(id: Uuid, db: &DbConn) -> Result<GParent, String> {
        let selected_parent = Parent::find_by_id(id).one(db).await;
        match selected_parent {
            Ok(parent_op) => match parent_op {
                Some(some_parent) => Ok(GParent {
                    id: some_parent.id,
                    first_name: some_parent.first_name,
                    last_name: some_parent.last_name,
                }),
                None => Err(String::from("parent doesnt exist")),
            },
            Err(e) => Err(e.to_string()),
        }
    }
    //
    pub async fn get_contact(id: Uuid, db: &DbConn) -> Result<GContact, String> {
        let selected_contact = Contact::find_by_id(id).one(db).await;
        match selected_contact {
            Ok(contact_op) => match contact_op {
                Some(some_contact) => Ok(GContact {
                    id: some_contact.id,
                    phone: some_contact.phone_number,
                    email: some_contact.email,
                    country_id: None,
                    state_id: None,
                    city_id: None,
                    district_id: None,
                    street_id: None,
                }),
                None => Err(String::from("contact doesnt exist")),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}
