use crate::Filters;
use sea_orm::Condition;

pub fn generate_student_filters(filters: Vec<Filters>) -> Condition {
    let conditions = Condition::all();
    for filter in filters {
        match filter.feild {
            _ => {}
        }
    }
    conditions
}
