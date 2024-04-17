pub mod models;
pub mod mutation;
pub mod query;
pub mod transaction;
pub mod utils;

// group imports from entity
mod entities {
    pub use ::entity::activities::{
        self, ActiveModel as ActivityActiveModel, Entity as Activities,
    };
    pub use ::entity::announcements::{
        self, ActiveModel as AnnouncementActiveModel, Entity as Announcements,
    };
    pub use ::entity::assignments::{
        self, ActiveModel as AssignmentActiveModel, Entity as Assignments,
    };
    pub use ::entity::classes::{self, ActiveModel as ClassActiveModel, Entity as Classes};
    pub use ::entity::disciplinary_actions::{
        self, ActiveModel as DisciplinaryActiveModel, Entity as Disciplinaries,
    };
    pub use ::entity::events::{self, ActiveModel as EventActiveModel, Entity as Events};
    pub use ::entity::grades::{self, ActiveModel as GradeActiveModel, Entity as Grades};
    pub use ::entity::grading_criteria::{ActiveModel as CriteriaActiveModel, Entity as Criterias};
    pub use ::entity::grading_rubrics::{
        self, ActiveModel as RubricActiveModel, Entity as Rubrics,
    };
    pub use ::entity::groups::{self, ActiveModel as GroupActiveModel, Entity as Groups};
    pub use ::entity::lectures::{self, ActiveModel as LectureActiveModel, Entity as Lectures};
    pub use ::entity::levels::{self, ActiveModel as LevelActiveModel, Entity as Levels};
    pub use ::entity::parents::{self, ActiveModel as ParentActiveModel, Entity as Parents};
    pub use ::entity::persons::{self, ActiveModel as PersonActiveModel, Entity as Persons};
    // pub use ::entity::pickups;
    pub use ::entity::rooms::{self, ActiveModel as RoomActiveModel, Entity as Rooms};
    pub use ::entity::scans::{self, ActiveModel as ScanActiveModel, Entity as Scans};
    pub use ::entity::sea_orm_active_enums::*;
    pub use ::entity::sessions::{self, ActiveModel as SessionActiveModel, Entity as Sessions};
    pub use ::entity::students::{self, ActiveModel as StudentActiveModel, Entity as Students};
    pub use ::entity::subjects::{self, ActiveModel as SubjectActiveModel, Entity as Subjects};
    pub use ::entity::teacher_subjects::{
        ActiveModel as TeacherSubjectActiveModel, Entity as TeacherSubjects,
    };
    pub use ::entity::teachers::{self, ActiveModel as TeacherActiveModel, Entity as Teachers};
    pub use ::entity::time_table::{
        self, ActiveModel as TimeTableActiveModel, Entity as TimeTables,
    };
    pub use ::entity::users::{self, ActiveModel as UserActiveModel, Entity as Users};
}

pub use chrono;
pub use sea_orm;
pub use uuid;
