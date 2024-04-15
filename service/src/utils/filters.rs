use crate::{
    entities::{parents, teachers},
    query::Filters,
};
use sea_orm::{ColumnTrait, Condition};

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
