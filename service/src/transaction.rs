use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set, TransactionError, TransactionTrait};

use super::{ParentWithAddress, StudentWithAddress, TeacherWithAddress};
use ::entity::contacts_informations::ActiveModel as ContactsActiveModel;
use ::entity::parents::ActiveModel as ParentsActiveModel;
use ::entity::persons::ActiveModel as PersonsActiveModel;
use ::entity::students::ActiveModel as StudentsActiveModel;
use ::entity::teachers::ActiveModel as TeachersActiveModel;

pub struct ServiceTransaction;

type TxnRes = Result<(), TransactionError<DbErr>>;

impl ServiceTransaction {
    pub async fn create_student(db: DbConn, data: StudentWithAddress) -> TxnRes {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create contact first
                let contact = ContactsActiveModel {
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
                let person = PersonsActiveModel {
                    contact_id: Set(Some(contact.id.unwrap())),
                    person_type: Set("student".to_owned()),
                    ..Default::default()
                }
                .save(txn)
                .await?;
                // create student
                let _student = StudentsActiveModel {
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
                let contact = ContactsActiveModel {
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
                let person = PersonsActiveModel {
                    contact_id: Set(Some(contact.id.unwrap())),
                    person_type: Set("parent".to_owned()),
                    ..Default::default()
                }
                .save(txn)
                .await?;
                // create student
                let _teacher = TeachersActiveModel {
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
                let contact = ContactsActiveModel {
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
                let person = PersonsActiveModel {
                    contact_id: Set(Some(contact.id.unwrap())),
                    person_type: Set("parent".to_owned()),
                    ..Default::default()
                }
                .save(txn)
                .await?;
                // create student
                let _parent = ParentsActiveModel {
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
    pub async fn create_contact(db: DbConn, data: ContactsActiveModel) -> TxnRes {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                let _contact = data.save(txn).await?;

                Ok(())
            })
        })
        .await
    }
}
