use sea_orm::{prelude::*, Set, TransactionError, TransactionTrait};

use crate::{
    entities::*,
    models::{Activity, Event, Lecture, Parent, Rubric, Student, Teacher, User},
    utils::convert_to_enum::{to_day_of_week, to_performance},
};

type TxnRes<T> = Result<T, TransactionError<DbErr>>;
pub struct TransactionService;

impl TransactionService {
    pub async fn create_student(db: DbConn, data: Student) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create person
                let person = PersonActiveModel {
                    person_type: Set(PersonTypeEnum::Student),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                //
                StudentActiveModel {
                    first_name: Set(data.first_name),
                    last_name: Set(data.last_name),
                    group_id: Set(data.group_id),
                    person_id: Set(Some(person.id)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                Ok(())
            })
        })
        .await
    }
    pub async fn create_teacher(db: DbConn, data: Teacher) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create person
                let person = PersonActiveModel {
                    person_type: Set(PersonTypeEnum::Teacher),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                //
                TeacherActiveModel {
                    first_name: Set(data.first_name),
                    last_name: Set(data.last_name),
                    person_id: Set(Some(person.id)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                Ok(())
            })
        })
        .await
    }
    pub async fn create_parent(db: DbConn, data: Parent) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create person
                let person = PersonActiveModel {
                    person_type: Set(PersonTypeEnum::Parent),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                //
                ParentActiveModel {
                    first_name: Set(data.first_name),
                    last_name: Set(data.last_name),
                    person_id: Set(Some(person.id)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                Ok(())
            })
        })
        .await
    }
    pub async fn upsert_user(db: &DbConn, data: User) -> TxnRes<Uuid> {
        db.transaction::<_, Uuid, DbErr>(|txn| {
            Box::pin(async move {
                // check if user exists
                let user = Users::find()
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
                    person_type: Set(PersonTypeEnum::NotDefined),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                let c_user = UserActiveModel {
                    first_name: Set(data.first_name),
                    last_name: Set(data.last_name),
                    email: Set(data.email),
                    picture: Set(data.picture),
                    person_id: Set(c_person.id),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                Ok(c_user.id)
            })
        })
        .await
    }
    pub async fn create_event(db: &DbConn, data: Event) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create time table
                let timetable_active_modal = TimeTableActiveModel {
                    item_type: Set(TimeTableItemTypeEnum::Lecture),
                    full_date: Set(Some(data.full_date)),
                    start_time: Set(Some(data.start_time)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                // create create event
                EventActiveModel {
                    time_table_id: Set(Some(timetable_active_modal.id)),
                    event_title: Set(Some(data.event_title)),
                    event_description: Set(Some(data.event_description)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                Ok(())
            })
        })
        .await
    }
    pub async fn create_activity(db: &DbConn, data: Activity) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create time table
                let timetable_active_modal = TimeTableActiveModel {
                    item_type: Set(TimeTableItemTypeEnum::Activity),
                    start_time: Set(Some(data.start_time)),
                    end_time: Set(Some(data.end_time)),
                    day_of_week: Set(to_day_of_week(data.day_of_week)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                // create create activity
                ActivityActiveModel {
                    time_table_id: Set(Some(timetable_active_modal.id)),
                    activity_title: Set(Some(data.activity_title)),
                    activity_description: Set(Some(data.activity_description)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                Ok(())
            })
        })
        .await
    }
    pub async fn create_lecture(db: &DbConn, data: Lecture) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create time table
                let timetable_active_modal = TimeTableActiveModel {
                    item_type: Set(TimeTableItemTypeEnum::Lecture),
                    start_time: Set(Some(data.start_time)),
                    end_time: Set(Some(data.end_time)),
                    day_of_week: Set(to_day_of_week(data.day_of_week)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                // create create lecture
                LectureActiveModel {
                    time_table_id: Set(Some(timetable_active_modal.id)),
                    class_id: Set(Some(data.class_id)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                Ok(())
            })
        })
        .await
    }
    //
    pub async fn create_rubric(db: &DbConn, data: Rubric) -> TxnRes<Uuid> {
        db.transaction::<_, Uuid, DbErr>(|txn| {
            Box::pin(async move {
                // create grading rubric
                let rubric_modal = RubricActiveModel {
                    title: Set(data.title),
                    description: Set(data.description),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                // create create criterias
                if let Some(grading_criterias) = data.grading_criterias {
                    let mut criterias = Vec::<CriteriaActiveModel>::new();
                    for critera in grading_criterias {
                        criterias.push(CriteriaActiveModel {
                            grading_rubric_id: Set(rubric_modal.id),
                            description: Set(critera.description),
                            performance: Set(to_performance(critera.performance)),
                            ..Default::default()
                        })
                    }
                    Criterias::insert_many(criterias).exec(txn).await?;
                };
                Ok(rubric_modal.id)
            })
        })
        .await
    }
}
