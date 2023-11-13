use sea_orm::Condition;

pub fn generate_filters() -> Condition {
    let conditions = Condition::all();

    conditions
}
