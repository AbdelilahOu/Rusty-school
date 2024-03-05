use super::{ParentWithAddress, StudentWithAddress, TeacherWithAddress};
use crate::CUser;
use ::entity::prelude::*;
use sea_orm::{prelude::Uuid, *};

pub struct TransactionsService;

type TxnRes<T> = Result<T, TransactionError<DbErr>>;

impl TransactionsService {
    pub async fn create_student(db: DbConn, data: StudentWithAddress) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create details first
                let details = PersonDetailsActiveModel {
                    email: Set(data.details.email),
                    phone_number: Set(data.details.phone),
                    country_id: Set(data.details.country_id),
                    state_id: Set(data.details.state_id),
                    city_id: Set(data.details.city_id),
                    street_id: Set(data.details.street_id),
                    ..Default::default()
                }
                .save(txn)
                .await?;
                // create person
                let person = PersonActiveModel {
                    details_id: Set(Some(details.id.unwrap())),
                    person_type: Set("student".to_owned()),
                    ..Default::default()
                }
                .save(txn)
                .await?;
                // create student
                let _student = StudentActiveModel {
                    first_name: Set(data.student.first_name),
                    last_name: Set(data.student.last_name),
                    group_id: Set(data.student.group_id),
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
    pub async fn create_teacher(db: DbConn, data: TeacherWithAddress) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create details first
                let details = PersonDetailsActiveModel {
                    email: Set(data.details.email),
                    phone_number: Set(data.details.phone),
                    country_id: Set(data.details.country_id),
                    state_id: Set(data.details.state_id),
                    city_id: Set(data.details.city_id),
                    street_id: Set(data.details.street_id),
                    ..Default::default()
                }
                .save(txn)
                .await?;
                // create person
                let person = PersonActiveModel {
                    details_id: Set(Some(details.id.unwrap())),
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
    pub async fn create_parent(db: DbConn, data: ParentWithAddress) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create details first
                let details = PersonDetailsActiveModel {
                    email: Set(data.details.email),
                    phone_number: Set(data.details.phone),
                    country_id: Set(data.details.country_id),
                    state_id: Set(data.details.state_id),
                    city_id: Set(data.details.city_id),
                    street_id: Set(data.details.street_id),
                    ..Default::default()
                }
                .save(txn)
                .await?;
                // create person
                let person = PersonActiveModel {
                    details_id: Set(Some(details.id.unwrap())),
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
    pub async fn upsert_user(db: &DbConn, data: CUser) -> TxnRes<Uuid> {
        db.transaction::<_, Uuid, DbErr>(|txn| {
            Box::pin(async move {
                // check if user exists
                let user = User::find()
                    .filter(users::Column::Email.eq(&data.email))
                    .one(txn)
                    .await?;

                if user.is_some() {
                    println!("user already exists");
                    // upsert the user first
                    return Ok(user.unwrap().id.to_owned());
                }
                // create details first
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

                Ok(_c_user.id.unwrap())
            })
        })
        .await
    }
    pub async fn create_event(db: &DbConn) -> TxnRes<Uuid> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create time table

                // create create event

                Ok(())
            })
        })
        .await;
        todo!()
    }
    pub async fn create_activity(db: &DbConn) -> TxnRes<Uuid> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create time table

                // create create activity

                Ok(())
            })
        })
        .await;
        todo!()
    }
    pub async fn create_lecture(db: &DbConn) -> TxnRes<Uuid> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create time table

                // create create lecture

                Ok(())
            })
        })
        .await;
        todo!()
    }
    //
}
