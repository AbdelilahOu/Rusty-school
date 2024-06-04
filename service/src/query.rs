use chrono::NaiveDateTime;
use sea_orm::{
    Condition,
    DbBackend,
    FromQueryResult, Iterable, JoinType, Order, prelude::*, QueryOrder, QuerySelect, sea_query::{Alias, Expr, extension::postgres::PgExpr, PostgresQueryBuilder, Query, SimpleExpr, SubQueryStatement}, Statement,
};

use crate::{
    entities::{
        activities, Activities, announcements, Announcements, assignments, Assignments, classes, Classes, disciplinary_actions, DisciplinaryActions, events, Events,
        grades, Grades, grading_rubrics, groups, Groups, lectures, Lectures, levels, Levels, parents, Parents, persons, Persons,
        rooms, Rooms, Rubrics, scans, Scans, sessions, Sessions, students, Students, subjects, Subjects, teachers, Teachers, time_table, TimeTableItemCategories,
        TimeTables, users, Users,
    },
    models::{
        AnnouncementQuery, AssignmentQuery, AttendanceQuery, ClassQuery, DisciplinaryQuery, GetSessionWithRole, GradeQuery, GroupQuery, LevelQuery,
        ParentQuery, RoomQuery, RubricQuery, ScansQuery, SelectAttendance, SelectScans, SelectTimeTable, StudentQuery, SubjectQuery, TeacherQuery,
    },
};

type Values = Vec<serde_json::Value>;
type DbResult<T> = Result<T, DbErr>;

pub struct QueryService;

impl QueryService {
    // students entity
    pub async fn list_students(db: &DbConn, query: StudentQuery) -> DbResult<Values> {
        let mut conditions = Condition::all();
        if let Some(name) = query.full_name {
            conditions = conditions.add(Expr::col(students::Column::FullName).ilike(name));
        }
        let students = Students::find()
            .select_only()
            .columns([
                students::Column::Id,
                students::Column::PersonId,
                students::Column::GroupId,
                students::Column::FullName,
            ])
            .expr(Expr::col(groups::Column::GroupName))
            .join(JoinType::LeftJoin, students::Relation::Groups.def())
            .filter(conditions)
            .order_by_desc(students::Column::CreatedAt)
            .offset((query.page - 1) * query.limit)
            .limit(query.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(students)
    }
    //
    pub async fn list_teachers(db: &DbConn, query: TeacherQuery) -> DbResult<Values> {
        let mut conditions = Condition::all();
        if let Some(name) = query.full_name {
            conditions = conditions.add(Expr::col(teachers::Column::FullName).ilike(name));
        }
        let teachers = Teachers::find()
            .select_only()
            .columns([teachers::Column::Id, teachers::Column::FullName])
            .expr(Expr::col(levels::Column::LevelName))
            .join(JoinType::LeftJoin, teachers::Relation::Levels.def())
            .filter(conditions)
            .offset((query.page - 1) * query.limit)
            .limit(query.limit)
            .order_by_desc(teachers::Column::CreatedAt)
            .into_json()
            .all(db)
            .await?;

        Ok(teachers)
    }
    //
    pub async fn list_parents(db: &DbConn, query: ParentQuery) -> DbResult<Values> {
        let mut conditions = Condition::all();
        if let Some(name) = query.full_name {
            conditions = conditions.add(Expr::col(teachers::Column::FullName).ilike(name));
        }
        let parents = Parents::find()
            .select_only()
            .columns([parents::Column::Id, parents::Column::FullName])
            .filter(conditions)
            .offset((query.page - 1) * query.limit)
            .limit(query.limit)
            .group_by(parents::Column::Id)
            .order_by_desc(parents::Column::CreatedAt)
            .into_json()
            .all(db)
            .await?;

        Ok(parents)
    }
    //
    pub async fn list_scans(db: &DbConn, query: ScansQuery) -> DbResult<Values> {
        //
        let (sql, values) = Query::select()
            .from(Scans)
            .exprs([
                Expr::col((Scans, scans::Column::Id)),
                Expr::col((Scans, scans::Column::PersonId)),
                Expr::col((Scans, scans::Column::ScanDate)),
                Expr::col((Persons, persons::Column::PersonType)),
            ])
            // GET full_name
            .expr_as(
                Expr::case(
                    Expr::col(persons::Column::PersonType).eq("student".to_owned()),
                    SimpleExpr::SubQuery(
                        None,
                        Box::new(SubQueryStatement::SelectStatement(
                            Query::select()
                                .from(Students)
                                .column(students::Column::FullName)
                                .cond_where(Expr::col((Students, students::Column::PersonId)).equals((Scans, scans::Column::PersonId)))
                                .to_owned(),
                        )),
                    ),
                )
                    .case(
                        Expr::col(persons::Column::PersonType).eq("parent".to_owned()),
                        SimpleExpr::SubQuery(
                            None,
                            Box::new(SubQueryStatement::SelectStatement(
                                Query::select()
                                    .from(Parents)
                                    .column(parents::Column::FullName)
                                    .cond_where(Expr::col((Parents, parents::Column::PersonId)).equals((Scans, scans::Column::PersonId)))
                                    .to_owned(),
                            )),
                        ),
                    )
                    .case(
                        Expr::col(persons::Column::PersonType).eq("teacher".to_owned()),
                        SimpleExpr::SubQuery(
                            None,
                            Box::new(SubQueryStatement::SelectStatement(
                                Query::select()
                                    .from(Teachers)
                                    .column(teachers::Column::FullName)
                                    .cond_where(Expr::col((Teachers, teachers::Column::PersonId)).equals((Scans, scans::Column::PersonId)))
                                    .to_owned(),
                            )),
                        ),
                    ),
                Alias::new("full_name"),
            )
            // GET _id
            .expr_as(
                Expr::case(
                    Expr::col(persons::Column::PersonType).eq("student".to_owned()),
                    SimpleExpr::SubQuery(
                        None,
                        Box::new(SubQueryStatement::SelectStatement(
                            Query::select()
                                .from(Students)
                                .column(students::Column::Id)
                                .cond_where(Expr::col((Students, students::Column::PersonId)).equals((Scans, scans::Column::PersonId)))
                                .to_owned(),
                        )),
                    ),
                )
                    .case(
                        Expr::col(persons::Column::PersonType).eq("parent".to_owned()),
                        SimpleExpr::SubQuery(
                            None,
                            Box::new(SubQueryStatement::SelectStatement(
                                Query::select()
                                    .from(Parents)
                                    .column(parents::Column::Id)
                                    .cond_where(Expr::col((Parents, parents::Column::PersonId)).equals((Scans, scans::Column::PersonId)))
                                    .to_owned(),
                            )),
                        ),
                    )
                    .case(
                        Expr::col(persons::Column::PersonType).eq("teacher".to_owned()),
                        SimpleExpr::SubQuery(
                            None,
                            Box::new(SubQueryStatement::SelectStatement(
                                Query::select()
                                    .from(Teachers)
                                    .column(teachers::Column::Id)
                                    .cond_where(Expr::col((Teachers, teachers::Column::PersonId)).equals((Scans, scans::Column::PersonId)))
                                    .to_owned(),
                            )),
                        ),
                    ),
                Alias::new("_id"),
            )
            //
            .join(
                JoinType::Join,
                Persons,
                Expr::col((Persons, persons::Column::Id)).equals((Scans, scans::Column::PersonId)),
            )
            // FULL_NAME filter
            .conditions(
                query.full_name.is_some(),
                |x| {
                    let full_name = query.full_name.unwrap();
                    x.and_where(
                        Expr::case(
                            Expr::col(persons::Column::PersonType).eq("student".to_owned()),
                            SimpleExpr::SubQuery(
                                None,
                                Box::new(SubQueryStatement::SelectStatement(
                                    Query::select()
                                        .from(Students)
                                        .column(students::Column::FullName)
                                        .cond_where(Expr::col((Students, students::Column::PersonId)).equals((Scans, scans::Column::PersonId)))
                                        .to_owned(),
                                )),
                            )
                                .ilike(format!("%{}%", full_name)),
                        )
                            .case(
                                Expr::col(persons::Column::PersonType).eq("parent".to_owned()),
                                SimpleExpr::SubQuery(
                                    None,
                                    Box::new(SubQueryStatement::SelectStatement(
                                        Query::select()
                                            .from(Parents)
                                            .column(parents::Column::FullName)
                                            .cond_where(Expr::col((Parents, parents::Column::PersonId)).equals((Scans, scans::Column::PersonId)))
                                            .to_owned(),
                                    )),
                                )
                                    .ilike(format!("%{}%", full_name)),
                            )
                            .case(
                                Expr::col(persons::Column::PersonType).eq("teacher".to_owned()),
                                SimpleExpr::SubQuery(
                                    None,
                                    Box::new(SubQueryStatement::SelectStatement(
                                        Query::select()
                                            .from(Teachers)
                                            .column(teachers::Column::FullName)
                                            .cond_where(Expr::col((Teachers, teachers::Column::PersonId)).equals((Scans, scans::Column::PersonId)))
                                            .to_owned(),
                                    )),
                                )
                                    .ilike(format!("%{}%", full_name)),
                            )
                            .into(),
                    );
                },
                |_| {},
            )
            // START scan_date
            .conditions(
                query.scan_time_start.is_some(),
                |x| {
                    // get value
                    let scan_time_start = query.scan_time_start.unwrap();
                    // check
                    let start_time = NaiveDateTime::parse_from_str(scan_time_start.as_str(), "%Y-%m-%d %H:%M:%S%");
                    // parse success
                    if let Ok(start_time) = start_time {
                        x.and_where(Expr::col((Scans, scans::Column::ScanDate)).gte(start_time));
                    } else {
                        println!("error parsing date : {:?}", start_time.err());
                    }
                },
                |_| {},
            )
            // END scan_date
            .conditions(
                query.scan_time_end.is_some(),
                |x| {
                    // get value
                    let scan_time_end = query.scan_time_end.unwrap();
                    // check
                    let end_time = NaiveDateTime::parse_from_str(scan_time_end.as_str(), "%Y-%m-%d %H:%M:%S%");
                    // parse success
                    if let Ok(end_time) = end_time {
                        x.and_where(Expr::col((Scans, scans::Column::ScanDate)).gte(end_time));
                    } else {
                        println!("error parsing date : {:?}", end_time.err());
                    }
                },
                |_| {},
            )
            // FILTER BY PERSON_TYPE
            .conditions(
                query.person_type.is_some(),
                |x| {
                    let person_type = query.person_type.unwrap();
                    x.and_where(Expr::col(persons::Column::PersonType).eq(person_type));
                },
                |_| {},
            )
            //
            .offset((query.page - 1) * query.limit)
            .limit(query.limit)
            .order_by(scans::Column::ScanDate, Order::Desc)
            .to_owned()
            .build(PostgresQueryBuilder);

        let result = SelectScans::find_by_statement(Statement::from_sql_and_values(DbBackend::Postgres, sql, values))
            .into_json()
            .all(db)
            .await?;

        Ok(result)
    }
    //
    pub async fn list_attendance(db: &DbConn, query: AttendanceQuery) -> DbResult<Values> {
        //
        let (sql, values) = Query::select()
            .from(Scans)
            .exprs([
                Expr::col((Scans, scans::Column::Id)),
                Expr::col((Scans, scans::Column::PersonId)),
                Expr::col((Scans, scans::Column::ScanDate)),
                Expr::col((Students, students::Column::FullName)),
                Expr::col((Groups, groups::Column::GroupName)),
                Expr::col((Levels, levels::Column::LevelName)),
            ])
            // GET _id
            .expr_as(Expr::col((Students, students::Column::Id)), Alias::new("_id"))
            //
            .join(
                JoinType::Join,
                Students,
                Expr::col((Students, students::Column::PersonId)).equals((Scans, scans::Column::PersonId)),
            )
            //
            .join(
                JoinType::Join,
                Groups,
                Expr::col((Groups, groups::Column::Id)).equals((Students, students::Column::GroupId)),
            )
            //
            .join(
                JoinType::Join,
                Levels,
                Expr::col((Levels, levels::Column::Id)).equals((Groups, groups::Column::LevelId)),
            )
            // FULL_NAME filter
            .conditions(
                query.full_name.is_some(),
                |x| {
                    let full_name = query.full_name.unwrap();
                    x.and_where(Expr::col(students::Column::FullName).ilike(format!("%{}%", full_name)));
                },
                |_| {},
            )
            // START scan_date
            .conditions(
                query.scan_time_start.is_some(),
                |x| {
                    let start_time = query.scan_time_start.unwrap();
                    // check
                    let start_time = NaiveDateTime::parse_from_str(start_time.as_str(), "%Y-%m-%d %H:%M:%S%");
                    // parse success
                    if let Ok(start_time) = start_time {
                        x.and_where(Expr::col((Scans, scans::Column::ScanDate)).gte(start_time));
                    } else {
                        println!("error parsing date : {:?}", start_time.err());
                    }
                },
                |_| {},
            )
            // END scan_date
            .conditions(
                query.scan_time_end.is_some(),
                |x| {
                    let end_time = query.scan_time_end.unwrap();
                    // check
                    let end_time = NaiveDateTime::parse_from_str(end_time.as_str(), "%Y-%m-%d %H:%M:%S%");
                    // parse success
                    if let Ok(end_time) = end_time {
                        x.and_where(Expr::col((Scans, scans::Column::ScanDate)).gte(end_time));
                    } else {
                        println!("error parsing date : {:?}", end_time.err());
                    }
                },
                |_| {},
            )
            // GROUP_ID
            .conditions(
                query.group_id.is_some(),
                |x| {
                    let group_id = query.group_id.unwrap();
                    x.and_where(Expr::col((Students, students::Column::GroupId)).eq(group_id));
                },
                |_| {},
            )
            //
            .offset((query.page - 1) * query.limit)
            .limit(query.limit)
            .order_by(scans::Column::ScanDate, Order::Desc)
            .to_owned()
            .build(PostgresQueryBuilder);

        let result = SelectAttendance::find_by_statement(Statement::from_sql_and_values(DbBackend::Postgres, sql, values))
            .into_json()
            .all(db)
            .await?;

        Ok(result)
    }
    //
    pub async fn list_levels(db: &DbConn, query: LevelQuery) -> DbResult<Values> {
        let mut conditions = Condition::all();
        if let Some(name) = query.name {
            conditions = conditions.add(Expr::col(levels::Column::LevelName).ilike(name));
        }
        let levels = Levels::find()
            .select_only()
            .columns([levels::Column::Id, levels::Column::LevelName, levels::Column::LevelDescription])
            .filter(conditions)
            .order_by_desc(levels::Column::CreatedAt)
            .offset((query.page - 1) * query.limit)
            .limit(query.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(levels)
    }
    //
    pub async fn list_subjects(db: &DbConn, query: SubjectQuery) -> DbResult<Values> {
        let level_alias = "level_name";
        let mut conditions = Condition::all();
        if let Some(name) = query.name {
            conditions = conditions.add(Expr::col(subjects::Column::SubjectName).ilike(name));
        }
        let subjects = Subjects::find()
            .select_only()
            .columns([
                subjects::Column::Id,
                subjects::Column::SubjectName,
                subjects::Column::SubjectDescription,
                subjects::Column::LevelId,
            ])
            .column_as(levels::Column::LevelName, level_alias)
            .join(JoinType::LeftJoin, subjects::Relation::Levels.def())
            .filter(conditions)
            .order_by_desc(subjects::Column::CreatedAt)
            .offset((query.page - 1) * query.limit)
            .limit(query.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(subjects)
    }
    //
    pub async fn list_level_subjects(db: &DbConn, level_id: Uuid) -> DbResult<Values> {
        let level_subjects = Subjects::find()
            .filter(subjects::Column::LevelId.eq(level_id.clone()))
            .into_json()
            .all(db)
            .await?;
        Ok(level_subjects)
    }
    //
    pub async fn list_groups(db: &DbConn, query: GroupQuery) -> DbResult<Values> {
        let mut conditions = Condition::all();
        if let Some(name) = query.name {
            conditions = conditions.add(Expr::col(groups::Column::GroupName).ilike(name));
        }
        let groups = Groups::find()
            .select_only()
            .columns([
                groups::Column::Id,
                groups::Column::GroupName,
                groups::Column::GroupDescription,
                groups::Column::LevelId,
            ])
            .expr(Expr::col(levels::Column::LevelName))
            .expr(Expr::col(levels::Column::LevelName))
            .join(JoinType::Join, groups::Relation::Levels.def())
            .filter(conditions)
            .order_by_desc(groups::Column::CreatedAt)
            .offset((query.page - 1) * query.limit)
            .limit(query.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(groups)
    }
    //
    pub async fn list_level_groups(db: &DbConn, level_id: Uuid) -> DbResult<Values> {
        let level_groups = Groups::find()
            .filter(groups::Column::LevelId.eq(level_id.clone()))
            .into_json()
            .all(db)
            .await?;
        Ok(level_groups)
    }
    //
    pub async fn list_rooms(db: &DbConn, query: RoomQuery) -> DbResult<Values> {
        let mut conditions = Condition::all();
        if let Some(name) = query.name {
            conditions = conditions.add(Expr::col(rooms::Column::RoomName).ilike(name));
        }
        let rooms = Rooms::find()
            .filter(conditions)
            .offset((query.page - 1) * query.limit)
            .limit(query.limit)
            .into_json()
            .all(db)
            .await?;
        Ok(rooms)
    }
    //
    pub async fn list_classes(db: &DbConn, query: ClassQuery) -> DbResult<Values> {
        let mut conditions = Condition::all();
        if let Some(teacher_id) = query.teacher_id {
            conditions = conditions.add(Expr::col(classes::Column::TeacherId).eq(teacher_id));
        }
        if let Some(group_id) = query.group_id {
            conditions = conditions.add(Expr::col(classes::Column::GroupId).eq(group_id));
        }
        if let Some(subject_id) = query.subject_id {
            conditions = conditions.add(Expr::col(classes::Column::SubjectId).eq(subject_id));
        }
        let classes = Classes::find()
            .filter(conditions)
            .offset((query.page - 1) * query.limit)
            .limit(query.limit)
            .into_json()
            .all(db)
            .await?;
        Ok(classes)
    }
    //
    pub async fn list_time_table(db: &DbConn) -> DbResult<Values> {
        let (sql, values) = Query::select()
            .from(TimeTables)
            .columns(time_table::Column::iter().filter(|f| match f {
                time_table::Column::DayOfWeek => false,
                time_table::Column::ItemType => false,
                _ => true,
            }))
            .expr_as(
                Expr::col((TimeTables, time_table::Column::DayOfWeek)).cast_as(Alias::new("TEXT")),
                Alias::new("day_of_week"),
            )
            .expr_as(
                Expr::col((TimeTables, time_table::Column::ItemType)).cast_as(Alias::new("TEXT")),
                Alias::new("item_type"),
            )
            .expr_as(
                Expr::case(
                    Expr::col(time_table::Column::ItemType)
                        .cast_as(Alias::new("TEXT"))
                        .eq(TimeTableItemCategories::Event),
                    SimpleExpr::SubQuery(
                        None,
                        Box::new(SubQueryStatement::SelectStatement(
                            Query::select()
                                .from(Events)
                                .column(events::Column::EventTitle)
                                .cond_where(Expr::col((Events, events::Column::TimeTableId)).equals((TimeTables, time_table::Column::Id)))
                                .to_owned(),
                        )),
                    ),
                )
                    .case(
                        Expr::col(time_table::Column::ItemType)
                            .cast_as(Alias::new("TEXT"))
                            .eq(TimeTableItemCategories::Activity),
                        SimpleExpr::SubQuery(
                            None,
                            Box::new(SubQueryStatement::SelectStatement(
                                Query::select()
                                    .from(Activities)
                                    .column(activities::Column::ActivityTitle)
                                    .cond_where(Expr::col((Activities, activities::Column::TimeTableId)).equals((TimeTables, time_table::Column::Id)))
                                    .to_owned(),
                            )),
                        ),
                    )
                    .finally(SimpleExpr::SubQuery(
                        None,
                        Box::new(SubQueryStatement::SelectStatement(
                            Query::select()
                                .from(Lectures)
                                .expr(Expr::cust(
                                    "FORMAT('%s, %s, %s', teachers.full_name, subjects.subject_name, groups.group_name)",
                                ))
                                .join(
                                    JoinType::Join,
                                    Classes,
                                    Expr::col((Classes, classes::Column::Id)).equals((Lectures, lectures::Column::ClassId)),
                                )
                                .join(
                                    JoinType::Join,
                                    Teachers,
                                    Expr::col((Teachers, classes::Column::Id)).equals((Classes, classes::Column::TeacherId)),
                                )
                                .join(
                                    JoinType::Join,
                                    Groups,
                                    Expr::col((Groups, classes::Column::Id)).equals((Classes, classes::Column::GroupId)),
                                )
                                .join(
                                    JoinType::Join,
                                    Subjects,
                                    Expr::col((Subjects, classes::Column::Id)).equals((Classes, classes::Column::SubjectId)),
                                )
                                .cond_where(Expr::col((Lectures, lectures::Column::TimeTableId)).equals((TimeTables, time_table::Column::Id)))
                                .to_owned(),
                        )),
                    )),
                Alias::new("title"),
            )
            .to_owned()
            .build(PostgresQueryBuilder);

        let result = SelectTimeTable::find_by_statement(Statement::from_sql_and_values(DbBackend::Postgres, sql, values))
            .into_json()
            .all(db)
            .await?;

        Ok(result)
    }
    //
    pub async fn list_assignments(db: &DbConn, query: AssignmentQuery) -> DbResult<Values> {
        let mut conditions = Condition::all();
        if let Some(teacher_id) = query.teacher_id {
            conditions = conditions.add(Expr::col(assignments::Column::TeacherId).eq(teacher_id));
        }
        if let Some(title) = query.title {
            conditions = conditions.add(Expr::col(assignments::Column::Title).eq(title));
        }
        if let Some(due_date) = query.due_date {
            conditions = conditions.add(Expr::col(assignments::Column::DueDate).eq(due_date));
        }
        let assignments = Assignments::find()
            .select_only()
            .columns(assignments::Column::iter().filter(|f| match f {
                assignments::Column::GradinRubricId => false,
                assignments::Column::File => false,
                _ => true,
            }))
            .filter(conditions)
            .offset((query.page - 1) * query.limit)
            .limit(query.limit)
            .into_json()
            .all(db)
            .await?;
        Ok(assignments)
    }
    //
    pub async fn list_grades(db: &DbConn, query: GradeQuery) -> DbResult<Values> {
        let mut conditions = Condition::all();
        if let Some(student_id) = query.student_id {
            conditions = conditions.add(Expr::col(grades::Column::StudentId).eq(student_id));
        }
        let grades = Grades::find()
            .select_only()
            .columns(grades::Column::iter())
            .filter(conditions)
            .offset((query.page - 1) * query.limit)
            .limit(query.limit)
            .into_json()
            .all(db)
            .await?;
        Ok(grades)
    }
    //
    pub async fn list_disciplinary_actions(db: &DbConn, query: DisciplinaryQuery) -> DbResult<Values> {
        let mut conditions = Condition::all();
        if let Some(student_id) = query.student_id {
            conditions = conditions.add(Expr::col(disciplinary_actions::Column::StudentId).eq(student_id));
        }
        if let Some(issued_at) = query.issued_at {
            conditions = conditions.add(Expr::col(disciplinary_actions::Column::IssuedAt).eq(issued_at));
        }
        let disciplinary_action = DisciplinaryActions::find()
            .select_only()
            .columns(disciplinary_actions::Column::iter())
            .column(students::Column::FullName)
            .join(JoinType::Join, disciplinary_actions::Relation::Students.def())
            .filter(conditions)
            .offset((query.page - 1) * query.limit)
            .limit(query.limit)
            .into_json()
            .all(db)
            .await?;
        Ok(disciplinary_action)
    }
    //
    pub async fn list_announcements(db: &DbConn, query: AnnouncementQuery) -> DbResult<Values> {
        let mut conditions = Condition::all();
        if let Some(title) = query.title {
            conditions = conditions.add(Expr::col(announcements::Column::Title).eq(title));
        }
        if let Some(start_date) = query.start_date {
            conditions = conditions.add(Expr::col(announcements::Column::StartDate).eq(start_date));
        }
        if let Some(end_date) = query.end_date {
            conditions = conditions.add(Expr::col(announcements::Column::EndDate).eq(end_date));
        }
        if let Some(category) = query.category {
            conditions = conditions.add(Expr::col(announcements::Column::Category).eq(category));
        }
        let announcements = Announcements::find()
            .select_only()
            .columns(announcements::Column::iter())
            .filter(conditions)
            .offset((query.page - 1) * query.limit)
            .limit(query.limit)
            .into_json()
            .all(db)
            .await?;
        Ok(announcements)
    }
    //
    pub async fn list_rubrics(db: &DbConn, query: RubricQuery) -> DbResult<Values> {
        let mut conditions = Condition::all();
        if let Some(title) = query.title {
            conditions = conditions.add(Expr::col(grading_rubrics::Column::Title).eq(title));
        }
        let rubrics = Rubrics::find()
            .select_only()
            .columns(grading_rubrics::Column::iter())
            .filter(conditions)
            .offset((query.page - 1) * query.limit)
            .limit(query.limit)
            .into_json()
            .all(db)
            .await?;
        Ok(rubrics)
    }
    //
    pub async fn get_session(db: &DbConn, id: Uuid) -> DbResult<Option<GetSessionWithRole>> {
        let (sql, values) = Query::select()
            .from(Sessions)
            .columns([
                (Sessions, sessions::Column::Id),
                (Sessions, sessions::Column::UserId),
                (Sessions, sessions::Column::IsBlocked),
                (Sessions, sessions::Column::RefreshToken),
                (Sessions, sessions::Column::ExpiresAt),
            ])
            .expr(Expr::col(users::Column::Role).cast_as(Alias::new("TEXT")))
            .join(
                JoinType::Join,
                Users,
                Expr::col((Users, users::Column::Id)).equals((Sessions, sessions::Column::UserId)),
            )
            .cond_where(Condition::all().add(Expr::col((Sessions, sessions::Column::Id)).eq(id)))
            .to_owned()
            .build(PostgresQueryBuilder);

        let result = GetSessionWithRole::find_by_statement(Statement::from_sql_and_values(DbBackend::Postgres, sql, values))
            .one(db)
            .await?;

        Ok(result)
    }
}
