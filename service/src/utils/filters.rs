use crate::Filters;
use entity::students::Column as StudentCol;
use entity::teachers::Column as TeacherCol;
use sea_orm::{ColumnTrait, Condition};

pub fn generate_student_filters(filters: Vec<Filters>) -> Condition {
    filters
        .into_iter()
        .fold(Condition::all(), |conditions, filter| {
            match filter.feild.as_str() {
                "full_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(StudentCol::FullName.like(filter.value)),
                    "CONTAINS" => conditions.add(StudentCol::FullName.contains(filter.value)),
                    "STARTS_WITH" => conditions.add(StudentCol::FullName.starts_with(filter.value)),
                    "ENDS_WITH" => conditions.add(StudentCol::FullName.ends_with(filter.value)),
                    _ => conditions,
                },
                "last_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(StudentCol::LastName.like(filter.value)),
                    "CONTAINS" => conditions.add(StudentCol::LastName.contains(filter.value)),
                    "STARTS_WITH" => conditions.add(StudentCol::LastName.starts_with(filter.value)),
                    "ENDS_WITH" => conditions.add(StudentCol::LastName.ends_with(filter.value)),
                    _ => conditions,
                },
                "first_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(StudentCol::FirstName.like(filter.value)),
                    "CONTAINS" => conditions.add(StudentCol::FirstName.contains(filter.value)),
                    "STARTS_WITH" => {
                        conditions.add(StudentCol::FirstName.starts_with(filter.value))
                    }
                    "ENDS_WITH" => conditions.add(StudentCol::FirstName.ends_with(filter.value)),
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
                "full_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(TeacherCol::FullName.like(filter.value)),
                    "CONTAINS" => conditions.add(TeacherCol::FullName.contains(filter.value)),
                    "STARTS_WITH" => conditions.add(TeacherCol::FullName.starts_with(filter.value)),
                    "ENDS_WITH" => conditions.add(TeacherCol::FullName.ends_with(filter.value)),
                    _ => conditions,
                },
                "last_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(TeacherCol::LastName.like(filter.value)),
                    "CONTAINS" => conditions.add(TeacherCol::LastName.contains(filter.value)),
                    "STARTS_WITH" => conditions.add(TeacherCol::LastName.starts_with(filter.value)),
                    "ENDS_WITH" => conditions.add(TeacherCol::LastName.ends_with(filter.value)),
                    _ => conditions,
                },
                "first_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(TeacherCol::FirstName.like(filter.value)),
                    "CONTAINS" => conditions.add(TeacherCol::FirstName.contains(filter.value)),
                    "STARTS_WITH" => {
                        conditions.add(TeacherCol::FirstName.starts_with(filter.value))
                    }
                    "ENDS_WITH" => conditions.add(TeacherCol::FirstName.ends_with(filter.value)),
                    _ => conditions,
                },
                _ => conditions,
            }
        })
}
