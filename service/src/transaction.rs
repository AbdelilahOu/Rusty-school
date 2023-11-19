use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set, TransactionTrait};

use ::entity::contacts_informations::ActiveModel as ContactsActiveModel;
use ::entity::persons::ActiveModel as PersonsActiveModel;
use ::entity::students::ActiveModel as StudentsActiveModel;

use crate::StudentWithAddress;

pub struct ServiceTransaction;

impl ServiceTransaction {
    pub async fn create_student(db: DbConn, data: StudentWithAddress) {
        let _ = db
            .transaction::<_, (), DbErr>(|txn| {
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
            .await;
    }
}
