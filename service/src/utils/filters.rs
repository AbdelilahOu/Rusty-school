use crate::Filters;
use entity::parents::Column as ParentCol;
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
                    _ => conditions,
                },
                "last_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(StudentCol::LastName.like(filter.value)),
                    _ => conditions,
                },
                "first_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(StudentCol::FirstName.like(filter.value)),
                    _ => conditions,
                },
                "level" => match filter.operation.as_str() {
                    "=" => {
                        // get level id using level
                        conditions
                    }
                    _ => conditions,
                },
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
                    _ => conditions,
                },
                "last_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(TeacherCol::LastName.like(filter.value)),
                    _ => conditions,
                },
                "first_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(TeacherCol::FirstName.like(filter.value)),
                    _ => conditions,
                },
                _ => conditions,
            }
        })
}

pub fn generate_parent_filters(filters: Vec<Filters>) -> Condition {
    filters
        .into_iter()
        .fold(Condition::all(), |conditions, filter| {
            match filter.feild.as_str() {
                "full_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(ParentCol::FullName.like(filter.value)),
                    _ => conditions,
                },
                "last_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(ParentCol::LastName.like(filter.value)),
                    _ => conditions,
                },
                "first_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(ParentCol::FirstName.like(filter.value)),
                    _ => conditions,
                },
                _ => conditions,
            }
        })
}
