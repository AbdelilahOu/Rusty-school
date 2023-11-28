use super::{ParentWithAddress, StudentWithAddress, TeacherWithAddress};
use crate::CUser;
use ::entity::prelude::*;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set, TransactionError, TransactionTrait};

pub struct ServiceTransaction;

type TxnRes = Result<(), TransactionError<DbErr>>;

impl ServiceTransaction {
    pub async fn create_student(db: DbConn, data: StudentWithAddress) -> TxnRes {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create contact first
                let contact = ContactActiveModel {
                    email: Set(data.contact.email),
                    phone_number: Set(data.contact.phone),
                    country_id: Set(data.contact.country_id),
                    state_id: Set(data.contact.state_id),
                    city_id: Set(data.contact.city_id),
                    street_id: Set(data.contact.street_id),
                    ..Default::default()
                }
                .save(txn)
                .await?;
                // create person
                let person = PersonActiveModel {
                    contact_id: Set(Some(contact.id.unwrap())),
                    person_type: Set("student".to_owned()),
                    ..Default::default()
                }
                .save(txn)
                .await?;
                // create student
                let _student = StudentActiveModel {
                    first_name: Set(data.student.first_name),
                    last_name: Set(data.student.last_name),
                    level: Set(data.student.level),
                    person_id: Set(Some(person.id.unwrap())),
                    ..Default::default()
                }
                .save(txn)
                .await?;

                Ok(())
            })
        })
        .await
    }
    pub async fn create_teacher(db: DbConn, data: TeacherWithAddress) -> TxnRes {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create contact first
                let contact = ContactActiveModel {
                    email: Set(data.contact.email),
                    phone_number: Set(data.contact.phone),
                    country_id: Set(data.contact.country_id),
                    state_id: Set(data.contact.state_id),
                    city_id: Set(data.contact.city_id),
                    street_id: Set(data.contact.street_id),
                    ..Default::default()
                }
                .save(txn)
                .await?;
                // create person
                let person = PersonActiveModel {
                    contact_id: Set(Some(contact.id.unwrap())),
                    person_type: Set("parent".to_owned()),
                    ..Default::default()
                }
                .save(txn)
                .await?;
                // create student
                let _teacher = TeacherActiveModel {
                    first_name: Set(data.teacher.first_name),
                    last_name: Set(data.teacher.last_name),
                    person_id: Set(Some(person.id.unwrap())),
                    ..Default::default()
                }
                .save(txn)
                .await?;

                Ok(())
            })
        })
        .await
    }
    pub async fn create_parent(db: DbConn, data: ParentWithAddress) -> TxnRes {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create contact first
                let contact = ContactActiveModel {
                    email: Set(data.contact.email),
                    phone_number: Set(data.contact.phone),
                    country_id: Set(data.contact.country_id),
                    state_id: Set(data.contact.state_id),
                    city_id: Set(data.contact.city_id),
                    street_id: Set(data.contact.street_id),
                    ..Default::default()
                }
                .save(txn)
                .await?;
                // create person
                let person = PersonActiveModel {
                    contact_id: Set(Some(contact.id.unwrap())),
                    person_type: Set("parent".to_owned()),
                    ..Default::default()
                }
                .save(txn)
                .await?;
                // create student
                let _parent = ParentActiveModel {
                    first_name: Set(data.parent.first_name),
                    last_name: Set(data.parent.last_name),
                    person_id: Set(Some(person.id.unwrap())),
                    ..Default::default()
                }
                .save(txn)
                .await?;

                Ok(())
            })
        })
        .await
    }
    pub async fn upsert_user(db: &DbConn, data: CUser) -> TxnRes {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                let c_person = PersonActiveModel {
                    person_type: Set("NOT DEFINED".to_owned()),
                    ..Default::default()
                }
                .save(txn)
                .await?;

                let _c_user = UserActiveModel {
                    first_name: Set(data.first_name),
                    last_name: Set(data.last_name),
                    email: Set(data.email),
                    picture: Set(data.picture),
                    person_id: Set(c_person.id.unwrap()),
                    ..Default::default()
                }
                .save(txn)
                .await?;

                Ok(())
            })
        })
        .await
    }
}
