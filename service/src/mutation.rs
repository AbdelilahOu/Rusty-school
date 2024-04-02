use chrono::Utc;
use sea_orm::{prelude::Uuid, *};

use super::models::*;
use crate::entities::*;

pub struct MutationsService;

type DyResult<T> = Result<T, DbErr>;

impl MutationsService {
    // students entity
    pub async fn create_student(db: &DbConn, data: CStudent) -> DyResult<Uuid> {
        let c_student = StudentActiveModel {
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            group_id: Set(data.group_id),
            ..Default::default()
        };
        let student = Student::insert(c_student).exec(db).await?;
        Ok(student.last_insert_id)
    }
    pub async fn delete_student(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let student_model = Student::find_by_id(id).one(db).await?;
        match student_model {
            Some(student_model) => {
                let student = student_model.delete(db).await?;
                Ok(student.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_student(db: &DbConn, id: Uuid, data: CStudent) -> DyResult<Uuid> {
        let student_model = Student::find_by_id(id).one(db).await?;
        match student_model {
            Some(student_model) => {
                let mut student_model: StudentActiveModel = student_model.into();
                // set new feild
                student_model.first_name = Set(data.first_name);
                student_model.last_name = Set(data.last_name);
                student_model.group_id = Set(data.group_id);
                student_model.id = Set(id);
                //
                let student = student_model.update(db).await?;
                Ok(student.id)
            }
            None => Ok(id),
        }
    }
    // teachers entity
    pub async fn create_teacher(db: &DbConn, data: CTeacher) -> DyResult<Uuid> {
        let teacher_model = TeacherActiveModel {
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            ..Default::default()
        };
        let teacher = Teacher::insert(teacher_model).exec(db).await?;
        Ok(teacher.last_insert_id)
    }
    pub async fn delete_teacher(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let teacher_model = Teacher::find_by_id(id).one(db).await?;
        match teacher_model {
            Some(teacher_model) => {
                let teacher = teacher_model.delete(db).await?;
                Ok(teacher.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_teacher(db: &DbConn, id: Uuid, data: CTeacher) -> DyResult<Uuid> {
        let teacher_model = Teacher::find_by_id(id).one(db).await?;
        match teacher_model {
            Some(teacher_model) => {
                //
                let mut teacher_model: TeacherActiveModel = teacher_model.into();
                teacher_model.first_name = Set(data.first_name);
                teacher_model.last_name = Set(data.last_name);
                teacher_model.id = Set(id);
                //
                let teacher = teacher_model.update(db).await?;
                Ok(teacher.id)
            }
            None => Ok(id),
        }
    }
    // parents entity
    pub async fn create_parent(db: &DbConn, data: CParent) -> DyResult<Uuid> {
        let parent_model = ParentActiveModel {
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            ..Default::default()
        };
        let parent = Parent::insert(parent_model).exec(db).await?;
        Ok(parent.last_insert_id)
    }
    pub async fn delete_parent(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let parent_model = Parent::find_by_id(id).one(db).await?;
        match parent_model {
            Some(parent_model) => {
                let parent = parent_model.delete(db).await?;
                Ok(parent.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_parent(db: &DbConn, id: Uuid, data: CParent) -> DyResult<Uuid> {
        let parent_model = Parent::find_by_id(id).one(db).await?;
        match parent_model {
            Some(parent_model) => {
                let mut parent_model: ParentActiveModel = parent_model.into();
                parent_model.first_name = Set(data.first_name);
                parent_model.last_name = Set(data.last_name);
                parent_model.id = Set(id);
                let parent = parent_model.update(db).await?;
                Ok(parent.id)
            }
            None => Ok(id),
        }
    }
    // scans
    pub async fn create_scan(db: &DbConn, data: CScan) -> DyResult<Uuid> {
        let now = Utc::now();
        let scan_model = ScanActiveModel {
            person_id: Set(data.person_id),
            scan_date: Set(now.naive_utc()),
            ..Default::default()
        };
        let scan = Scans::insert(scan_model).exec(db).await?;
        Ok(scan.last_insert_id)
    }
    //
    pub async fn create_level(db: &DbConn, data: CLevel) -> DyResult<Uuid> {
        let level_model = LevelActiveModel {
            level_name: Set(data.name),
            level_description: Set(data.description),
            ..Default::default()
        };
        let level = Level::insert(level_model).exec(db).await?;
        Ok(level.last_insert_id)
    }
    pub async fn delete_level(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let level_model = Level::find_by_id(id).one(db).await?;
        match level_model {
            Some(level_model) => {
                let level = level_model.delete(db).await?;
                Ok(level.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_level(db: &DbConn, id: Uuid, data: CLevel) -> DyResult<Uuid> {
        let level_model = Level::find_by_id(id).one(db).await?;
        match level_model {
            Some(level_model) => {
                //
                let mut level_model: LevelActiveModel = level_model.into();
                level_model.level_name = Set(data.name);
                level_model.level_description = Set(data.description);
                //
                let level = level_model.update(db).await?;
                Ok(level.id)
            }
            None => Ok(id),
        }
    }
    //
    pub async fn create_subject(db: &DbConn, data: CSubject) -> DyResult<Uuid> {
        let subject_model = SubjectActiveModel {
            subject_name: Set(data.name),
            subject_description: Set(data.description),
            level_id: Set(data.level_id),
            ..Default::default()
        };
        let subject = Subject::insert(subject_model).exec(db).await?;
        Ok(subject.last_insert_id)
    }
    pub async fn delete_subject(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let subject_model = Subject::find_by_id(id).one(db).await?;
        match subject_model {
            Some(subject_model) => {
                let subject = subject_model.delete(db).await?;
                Ok(subject.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_subject(db: &DbConn, id: Uuid, data: CSubject) -> DyResult<Uuid> {
        let subject_model = Subject::find_by_id(id).one(db).await?;
        match subject_model {
            Some(subject_model) => {
                //
                let mut subject_model: SubjectActiveModel = subject_model.into();
                subject_model.subject_name = Set(data.name);
                subject_model.subject_description = Set(data.description);
                subject_model.level_id = Set(data.level_id);
                //
                let subject = subject_model.update(db).await?;
                Ok(subject.id)
            }
            None => Ok(id),
        }
    }
    //
    pub async fn create_group(db: &DbConn, data: CGroup) -> DyResult<Uuid> {
        let group_model = GroupActiveModel {
            group_name: Set(data.name),
            group_description: Set(data.description),
            level_id: Set(data.level_id),
            ..Default::default()
        };
        let group = Group::insert(group_model).exec(db).await?;
        Ok(group.last_insert_id)
    }
    pub async fn delete_group(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let group_model = Group::find_by_id(id).one(db).await?;
        match group_model {
            Some(group_model) => {
                let group = group_model.delete(db).await?;
                Ok(group.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_group(db: &DbConn, id: Uuid, data: CGroup) -> DyResult<Uuid> {
        let group_model = Group::find_by_id(id).one(db).await?;
        match group_model {
            Some(group_model) => {
                //
                let mut group_model: GroupActiveModel = group_model.into();
                group_model.group_name = Set(data.name);
                group_model.group_description = Set(data.description);
                group_model.level_id = Set(data.level_id);
                //
                let group = group_model.update(db).await?;
                Ok(group.id)
            }
            None => Ok(id),
        }
    }
    //
    pub async fn create_room(db: &DbConn, data: CRoom) -> DyResult<Uuid> {
        let room_model = RoomActiveModel {
            room_name: Set(data.name),
            room_description: Set(data.description),
            ..Default::default()
        };
        let room = Room::insert(room_model).exec(db).await?;
        Ok(room.last_insert_id)
    }
    pub async fn delete_room(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let room_model = Room::find_by_id(id).one(db).await?;
        match room_model {
            Some(room_model) => {
                let room = room_model.delete(db).await?;
                Ok(room.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_room(db: &DbConn, id: Uuid, data: CRoom) -> DyResult<Uuid> {
        let room_model = Room::find_by_id(id).one(db).await?;
        match room_model {
            Some(room_model) => {
                //
                let mut room_model: RoomActiveModel = room_model.into();
                room_model.room_name = Set(data.name);
                room_model.room_description = Set(data.description);
                //
                let room = room_model.update(db).await?;
                Ok(room.id)
            }
            None => Ok(id),
        }
    }
    //
    pub async fn create_class(db: &DbConn, data: CClass) -> DyResult<Uuid> {
        let class_model = ClassActiveModel {
            subject_id: Set(data.subject_id),
            teacher_id: Set(data.teacher_id),
            group_id: Set(data.group_id),
            room_id: Set(data.room_id),
            ..Default::default()
        };
        let class = Class::insert(class_model).exec(db).await?;
        Ok(class.last_insert_id)
    }
    pub async fn delete_class(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let class_model = Class::find_by_id(id).one(db).await?;
        match class_model {
            Some(class_model) => {
                let class = class_model.delete(db).await?;
                Ok(class.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_class(db: &DbConn, id: Uuid, data: CClass) -> DyResult<Uuid> {
        let class_model = Class::find_by_id(id).one(db).await?;
        match class_model {
            Some(class_model) => {
                //
                let mut class_model: ClassActiveModel = class_model.into();
                class_model.subject_id = Set(data.subject_id);
                class_model.teacher_id = Set(data.teacher_id);
                class_model.group_id = Set(data.group_id);
                class_model.room_id = Set(data.room_id);
                //
                let class = class_model.update(db).await?;
                Ok(class.id)
            }
            None => Ok(id),
        }
    }
    //
    pub async fn delete_time_table(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let time_table_model = TimeTable::find_by_id(id).one(db).await?;
        match time_table_model {
            Some(time_table_model) => {
                let time_table = time_table_model.delete(db).await?;
                Ok(time_table.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
    pub async fn create_teacher_subject(db: &DbConn, data: (Uuid, Uuid)) -> DyResult<Uuid> {
        let teacher_subject_model = TeacherSubjectActiveModel {
            teacher_id: Set(Some(data.0)),
            subject_id: Set(Some(data.1)),
            ..Default::default()
        };
        let teacher_subject = TeacherSubject::insert(teacher_subject_model)
            .exec(db)
            .await?;
        Ok(teacher_subject.last_insert_id)
    }

    pub async fn delete_teacher_subject(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let teacher_subject_model = TeacherSubject::find_by_id(id).one(db).await?;
        match teacher_subject_model {
            Some(teacher_subject_model) => {
                let teacher_subject = teacher_subject_model.delete(db).await?;
                Ok(teacher_subject.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
    pub async fn create_assignment(db: &DbConn, data: CAssignment) -> DyResult<Uuid> {
        let assignment_model = AssignmentActiveModel {
            title: Set(data.title),
            description: Set(data.description),
            due_date: Set(data.due_date),
            submission_type: Set(data.submission_type),
            gradin_rubric_id: Set(data.gradin_rubric_id),
            file: Set(data.file),
            teacher_id: Set(data.teacher_id),
            ..Default::default()
        };
        let assignment = Assignment::insert(assignment_model).exec(db).await?;
        Ok(assignment.last_insert_id)
    }
    pub async fn update_assignment(db: &DbConn, id: Uuid, data: CAssignment) -> DyResult<Uuid> {
        let assignment_model = Assignment::find_by_id(id).one(db).await?;
        match assignment_model {
            Some(assignment_model) => {
                //
                let mut assignment_model: AssignmentActiveModel = assignment_model.into();
                assignment_model.title = Set(data.title);
                assignment_model.description = Set(data.description);
                assignment_model.due_date = Set(data.due_date);
                assignment_model.submission_type = Set(data.submission_type);
                assignment_model.gradin_rubric_id = Set(data.gradin_rubric_id);
                assignment_model.file = Set(data.file);
                assignment_model.teacher_id = Set(data.teacher_id);
                //
                let assignment = assignment_model.update(db).await?;
                Ok(assignment.id)
            }
            None => Ok(id),
        }
    }
    pub async fn delete_assignment(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let assignment_model = Assignment::find_by_id(id).one(db).await?;
        match assignment_model {
            Some(assignment_model) => {
                let assignment = assignment_model.delete(db).await?;
                Ok(assignment.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
    pub async fn create_grade(db: &DbConn, data: CGrade) -> DyResult<Uuid> {
        let grade_model = GradeActiveModel {
            student_id: Set(data.student_id),
            assignment_id: Set(data.assignment_id),
            feedback: Set(data.feedback),
            score: Set(data.score),
            ..Default::default()
        };
        let grade = Grade::insert(grade_model).exec(db).await?;
        Ok(grade.last_insert_id)
    }
    pub async fn update_grade(db: &DbConn, id: Uuid, data: CGrade) -> DyResult<Uuid> {
        let grade_model = Grade::find_by_id(id).one(db).await?;
        match grade_model {
            Some(grade_model) => {
                //
                let mut grade_model: GradeActiveModel = grade_model.into();
                grade_model.student_id = Set(data.student_id);
                grade_model.feedback = Set(data.feedback);
                grade_model.assignment_id = Set(data.assignment_id);
                grade_model.score = Set(data.score);
                //
                let grade = grade_model.update(db).await?;
                Ok(grade.id)
            }
            None => Ok(id),
        }
    }
    pub async fn delete_grade(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let grade_model = Grade::find_by_id(id).one(db).await?;
        match grade_model {
            Some(grade_model) => {
                let grade = grade_model.delete(db).await?;
                Ok(grade.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
    pub async fn create_disciplinary(db: &DbConn, data: CDisciAction) -> DyResult<Uuid> {
        let disciplinary_model = DisciplinaryActiveModel {
            student_id: Set(data.student_id),
            issued_at: Set(data.issued_at),
            description: Set(data.description),
            consequences: Set(data.consequences),
            ..Default::default()
        };
        let disciplinary = Disciplinary::insert(disciplinary_model).exec(db).await?;
        Ok(disciplinary.last_insert_id)
    }
    pub async fn update_disciplinary(db: &DbConn, id: Uuid, data: CDisciAction) -> DyResult<Uuid> {
        let disciplinary_model = Disciplinary::find_by_id(id).one(db).await?;
        match disciplinary_model {
            Some(disciplinary_model) => {
                //
                let mut disciplinary_model: DisciplinaryActiveModel = disciplinary_model.into();
                disciplinary_model.student_id = Set(data.student_id);
                disciplinary_model.issued_at = Set(data.issued_at);
                disciplinary_model.description = Set(data.description);
                disciplinary_model.consequences = Set(data.consequences);
                //
                let disciplinary = disciplinary_model.update(db).await?;
                Ok(disciplinary.id)
            }
            None => Ok(id),
        }
    }
    pub async fn delete_disciplinary(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let disciplinary_model = Disciplinary::find_by_id(id).one(db).await?;
        match disciplinary_model {
            Some(disciplinary_model) => {
                let disciplinary = disciplinary_model.delete(db).await?;
                Ok(disciplinary.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
    pub async fn create_rubric(db: &DbConn, data: CGrade) -> DyResult<Uuid> {
        let rubric_model = GradeActiveModel {
            student_id: Set(data.student_id),
            assignment_id: Set(data.assignment_id),
            feedback: Set(data.feedback),
            score: Set(data.score),
            ..Default::default()
        };
        let rubric = Grade::insert(rubric_model).exec(db).await?;
        Ok(rubric.last_insert_id)
    }
    pub async fn update_rubric(db: &DbConn, id: Uuid, data: CGrade) -> DyResult<Uuid> {
        let rubric_model = Grade::find_by_id(id).one(db).await?;
        match rubric_model {
            Some(rubric_model) => {
                //
                let mut rubric_model: GradeActiveModel = rubric_model.into();
                rubric_model.student_id = Set(data.student_id);
                rubric_model.feedback = Set(data.feedback);
                rubric_model.assignment_id = Set(data.assignment_id);
                rubric_model.score = Set(data.score);
                //
                let rubric = rubric_model.update(db).await?;
                Ok(rubric.id)
            }
            None => Ok(id),
        }
    }
    pub async fn delete_rubric(db: &DbConn, id: Uuid) -> DyResult<u64> {
        let rubric_model = Grade::find_by_id(id).one(db).await?;
        match rubric_model {
            Some(rubric_model) => {
                let rubric = rubric_model.delete(db).await?;
                Ok(rubric.rows_affected)
            }
            None => Ok(0),
        }
    }
}
