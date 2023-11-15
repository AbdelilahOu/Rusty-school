use crate::Filters;
use entity::students::Column as StudentColumn;
use entity::teachers::Column as TeacherColumn;
use sea_orm::{ColumnTrait, Condition};

pub fn generate_student_filters(filters: Vec<Filters>) -> Condition {
    filters
        .into_iter()
        .fold(Condition::all(), |conditions, filter| {
            match filter.feild.as_str() {
                "full_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(StudentColumn::FullName.like(filter.value)),
                    "CONTAINS" => conditions.add(StudentColumn::FullName.contains(filter.value)),
                    "STARTS_WITH" => {
                        conditions.add(StudentColumn::FullName.starts_with(filter.value))
                    }
                    "ENDS_WITH" => conditions.add(StudentColumn::FullName.ends_with(filter.value)),
                    _ => conditions,
                },
                "last_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(StudentColumn::LastName.like(filter.value)),
                    "CONTAINS" => conditions.add(StudentColumn::LastName.contains(filter.value)),
                    "STARTS_WITH" => {
                        conditions.add(StudentColumn::LastName.starts_with(filter.value))
                    }
                    "ENDS_WITH" => conditions.add(StudentColumn::LastName.ends_with(filter.value)),
                    _ => conditions,
                },
                "first_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(StudentColumn::FirstName.like(filter.value)),
                    "CONTAINS" => conditions.add(StudentColumn::FirstName.contains(filter.value)),
                    "STARTS_WITH" => {
                        conditions.add(StudentColumn::FirstName.starts_with(filter.value))
                    }
                    "ENDS_WITH" => conditions.add(StudentColumn::FirstName.ends_with(filter.value)),
                    _ => conditions,
                },
                "level" => conditions,
                "address" => conditions,
                _ => conditions,
            }
        })
}

pub fn generate_teacher_filters(filters: Vec<Filters>) -> Condition {
    filters
        .into_iter()
        .fold(Condition::all(), |conditions, filter| {
            match filter.feild.as_str() {
                // "full_name" => match filter.operation.as_str() {
                //     "LIKE" => conditions.add(TeacherColumn::FullName.like(filter.value)),
                //     "CONTAINS" => conditions.add(TeacherColumn::FullName.contains(filter.value)),
                //     "STARTS_WITH" => {
                //         conditions.add(TeacherColumn::FullName.starts_with(filter.value))
                //     }
                //     "ENDS_WITH" => conditions.add(TeacherColumn::FullName.ends_with(filter.value)),
                //     _ => conditions,
                // },
                "last_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(TeacherColumn::LastName.like(filter.value)),
                    "CONTAINS" => conditions.add(TeacherColumn::LastName.contains(filter.value)),
                    "STARTS_WITH" => {
                        conditions.add(TeacherColumn::LastName.starts_with(filter.value))
                    }
                    "ENDS_WITH" => conditions.add(TeacherColumn::LastName.ends_with(filter.value)),
                    _ => conditions,
                },
                "first_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(TeacherColumn::FirstName.like(filter.value)),
                    "CONTAINS" => conditions.add(TeacherColumn::FirstName.contains(filter.value)),
                    "STARTS_WITH" => {
                        conditions.add(TeacherColumn::FirstName.starts_with(filter.value))
                    }
                    "ENDS_WITH" => conditions.add(TeacherColumn::FirstName.ends_with(filter.value)),
                    _ => conditions,
                },
                _ => conditions,
            }
        })
}
