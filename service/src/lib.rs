pub mod models;
pub mod mutation;
pub mod query;
pub mod transaction;
pub mod utils;

// group imports from entity
mod entities {
    pub use ::entity::activities::{self, ActiveModel as ActivityActiveModel, Entity as Activity};
    pub use ::entity::announcements::{
        self, ActiveModel as AnnouncementActiveModel, Entity as Announcement,
    };
    pub use ::entity::assignments::{
        self, ActiveModel as AssignmentActiveModel, Entity as Assignment,
    };
    pub use ::entity::classes::{self, ActiveModel as ClassActiveModel, Entity as Class};
    pub use ::entity::disciplinary_actions::{
        self, ActiveModel as DisciplinaryActiveModel, Entity as Disciplinary,
    };
    pub use ::entity::events::{self, ActiveModel as EventActiveModel, Entity as Event};
    pub use ::entity::grades::{self, ActiveModel as GradeActiveModel, Entity as Grade};
    pub use ::entity::grading_rubrics::{ActiveModel as RubricActiveModel, Entity as Rubric};
    pub use ::entity::groups::{self, ActiveModel as GroupActiveModel, Entity as Group};
    pub use ::entity::lectures::{self, ActiveModel as LectureActiveModel, Entity as Lecture};
    pub use ::entity::levels::{self, ActiveModel as LevelActiveModel, Entity as Level};
    pub use ::entity::parents::{self, ActiveModel as ParentActiveModel, Entity as Parent};
    pub use ::entity::persons::{self, ActiveModel as PersonActiveModel, Entity as Persons};
    pub use ::entity::pickups;
    pub use ::entity::rooms::{ActiveModel as RoomActiveModel, Entity as Room};
    pub use ::entity::scans::{self, ActiveModel as ScanActiveModel, Entity as Scans};
    pub use ::entity::sea_orm_active_enums::*;
    pub use ::entity::students::{self, ActiveModel as StudentActiveModel, Entity as Student};
    pub use ::entity::subjects::{self, ActiveModel as SubjectActiveModel, Entity as Subject};
    pub use ::entity::teacher_subjects::{
        ActiveModel as TeacherSubjectActiveModel, Entity as TeacherSubject,
    };
    pub use ::entity::teachers::{self, ActiveModel as TeacherActiveModel, Entity as Teacher};
    pub use ::entity::time_table::{
        self, ActiveModel as TimeTableActiveModel, Entity as TimeTable,
    };
    pub use ::entity::users::{self, ActiveModel as UserActiveModel, Entity as User};
}

pub use chrono;
pub use sea_orm;
pub use uuid;
