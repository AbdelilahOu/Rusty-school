use crate::Filters;
use entity::students;
use sea_orm::{ColumnTrait, Condition};

pub fn generate_student_filters(filters: Vec<Filters>) -> Condition {
    filters
        .into_iter()
        .fold(Condition::all(), |conditions, filter| {
            match filter.feild.as_str() {
                "full_name" => conditions.add(students::Column::FirstName.like(filter.value)),
                "level" => conditions,
                "address" => conditions,
                _ => conditions,
            }
        })
}
