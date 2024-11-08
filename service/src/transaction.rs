use sea_orm::{prelude::*, Set, TransactionError, TransactionTrait};

use crate::{
    entities::{
        users, ActivityActiveModel, Criteria, CriteriaActiveModel, EventActiveModel,
        LectureActiveModel, ParentActiveModel, PersonActiveModel, PersonEnums, RubricActiveModel,
        StudentActiveModel, TeacherActiveModel, TimeTableActiveModel, TimeTableItemCategories,
        UserActiveModel, UserModel, Users, WeekDayNumber,
    },
    models::{Activity, Event, Lecture, Parent, Rubric, Student, Teacher, User},
};

type TxnRes<T> = Result<T, TransactionError<DbErr>>;

pub struct TransactionService;

impl TransactionService {
    pub async fn create_student(db: DbConn, data: Student) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create person
                let person = PersonActiveModel {
                    person_type: Set(PersonEnums::Student),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                //
                StudentActiveModel {
                    first_name: Set(data.first_name.unwrap()),
                    last_name: Set(data.last_name.unwrap()),
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
                    person_type: Set(PersonEnums::Teacher),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                //
                TeacherActiveModel {
                    first_name: Set(data.first_name.unwrap()),
                    last_name: Set(data.last_name.unwrap()),
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
                    person_type: Set(PersonEnums::Parent),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                //
                ParentActiveModel {
                    first_name: Set(data.first_name.unwrap()),
                    last_name: Set(data.last_name.unwrap()),
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
    pub async fn upsert_user(db: &DbConn, data: User) -> TxnRes<(User, Uuid)> {
        db.transaction::<_, (User, Uuid), DbErr>(|txn| {
            Box::pin(async move {
                // check if user exists
                let user_op: Option<UserModel> = Users::find()
                    .filter(users::Column::Email.eq(&data.email))
                    .one(txn)
                    .await?;

                if user_op.is_some() {
                    println!("user already exists");
                    // upsert the user first
                    let user = user_op.unwrap();
                    return Ok((
                        User {
                            name: user.name,
                            given_name: user.given_name,
                            family_name: user.family_name,
                            email: user.email,
                            picture: user.picture,
                            role: user.role.into(),
                        },
                        user.id,
                    ));
                }
                // create details first
                let c_person = PersonActiveModel {
                    person_type: Set(PersonEnums::NotDefined),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                let new_user: UserModel = UserActiveModel {
                    name: Set(data.name),
                    given_name: Set(data.given_name),
                    family_name: Set(data.family_name),
                    email: Set(data.email),
                    picture: Set(data.picture),
                    person_id: Set(c_person.id),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                Ok((
                    User {
                        name: new_user.name,
                        given_name: new_user.given_name,
                        family_name: new_user.family_name,
                        email: new_user.email,
                        picture: new_user.picture,
                        role: new_user.role.into(),
                    },
                    new_user.id,
                ))
            })
        })
        .await
    }
    pub async fn create_event(db: &DbConn, data: Event) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create timetable
                let timetable_active_modal = TimeTableActiveModel {
                    item_type: Set(TimeTableItemCategories::Lecture),
                    full_date: Set(Some(data.full_date)),
                    start_time: Set(Some(data.start_time)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                // create event
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
                // create timetable
                let timetable_active_modal = TimeTableActiveModel {
                    item_type: Set(TimeTableItemCategories::Activity),
                    start_time: Set(Some(data.start_time)),
                    end_time: Set(Some(data.end_time)),
                    day_of_week: Set(WeekDayNumber(data.day_of_week).into()),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                // create activity
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
                // create timetable
                let timetable_active_modal = TimeTableActiveModel {
                    item_type: Set(TimeTableItemCategories::Lecture),
                    start_time: Set(Some(data.start_time)),
                    end_time: Set(Some(data.end_time)),
                    day_of_week: Set(WeekDayNumber(data.day_of_week).into()),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                // create lecture
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
                // create criteria
                if let Some(grading_criteria) = data.grading_criterias {
                    let mut criteria_arr = Vec::<CriteriaActiveModel>::new();
                    for criteria in grading_criteria {
                        criteria_arr.push(CriteriaActiveModel {
                            grading_rubric_id: Set(rubric_modal.id),
                            description: Set(criteria.description),
                            performance: Set(criteria.performance.into()),
                            ..Default::default()
                        })
                    }
                    Criteria::insert_many(criteria_arr).exec(txn).await?;
                };
                Ok(rubric_modal.id)
            })
        })
        .await
    }
}
