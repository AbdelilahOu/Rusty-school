pub use sea_orm_migration::prelude::*;

mod m20231108_222103_c_teacher;
mod m20231109_190937_c_student;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231108_222103_c_teacher::Migration),
            Box::new(m20231109_190937_c_student::Migration),
        ]
    }
}
