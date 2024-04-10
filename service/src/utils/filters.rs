use crate::{
    entities::{parents, students, teachers},
    query::Filters,
};
use sea_orm::{ColumnTrait, Condition};

pub fn generate_student_filters(filters: Vec<Filters>) -> Condition {
    filters
        .into_iter()
        .fold(Condition::all(), |conditions, filter| {
            match filter.feild.as_str() {
                "full_name" => conditions.add(students::Column::FullName.contains(filter.value)),
                "group_id" => conditions.add(students::Column::GroupId.eq(filter.value)),
                _ => conditions,
            }
        })
}

pub fn generate_teacher_filters(filters: Vec<Filters>) -> Condition {
    filters
        .into_iter()
        .fold(Condition::all(), |conditions, filter| {
            match filter.feild.as_str() {
                "full_name" => conditions.add(teachers::Column::FullName.contains(filter.value)),
                _ => conditions,
            }
        })
}

pub fn generate_parent_filters(filters: Vec<Filters>) -> Condition {
    filters
        .into_iter()
        .fold(Condition::all(), |conditions, filter| {
            match filter.feild.as_str() {
                "full_name" => conditions.add(parents::Column::FullName.contains(filter.value)),
                _ => conditions,
            }
        })
}
