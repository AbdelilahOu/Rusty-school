use crate::Filters;
use entity::students::Column;
use sea_orm::{ColumnTrait, Condition};

pub fn generate_student_filters(filters: Vec<Filters>) -> Condition {
    filters
        .into_iter()
        .fold(Condition::all(), |conditions, filter| {
            match filter.feild.as_str() {
                "full_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(Column::FullName.like(filter.value)),
                    "CONTAINS" => conditions.add(Column::FullName.contains(filter.value)),
                    "STARTS_WITH" => conditions.add(Column::FullName.starts_with(filter.value)),
                    "ENDS_WITH" => conditions.add(Column::FullName.ends_with(filter.value)),
                    _ => conditions,
                },
                "last_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(Column::LastName.like(filter.value)),
                    "CONTAINS" => conditions.add(Column::LastName.contains(filter.value)),
                    "STARTS_WITH" => conditions.add(Column::LastName.starts_with(filter.value)),
                    "ENDS_WITH" => conditions.add(Column::LastName.ends_with(filter.value)),
                    _ => conditions,
                },
                "first_name" => match filter.operation.as_str() {
                    "LIKE" => conditions.add(Column::FirstName.like(filter.value)),
                    "CONTAINS" => conditions.add(Column::FirstName.contains(filter.value)),
                    "STARTS_WITH" => conditions.add(Column::FirstName.starts_with(filter.value)),
                    "ENDS_WITH" => conditions.add(Column::FirstName.ends_with(filter.value)),
                    _ => conditions,
                },
                "level" => conditions,
                "address" => conditions,
                _ => conditions,
            }
        })
}
