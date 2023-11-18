pub use sea_orm_migration::prelude::*;

mod m20231109_190937_c_student;
mod m20231113_170500_c_teacher;
mod m20231116_165911_c_parents;
mod m20231116_171406_c_pickups;
mod m20231116_214011_c_scans;
mod m20231116_234108_c_subject;
mod m20231117_132237_c_level;
mod m20231118_095513_c_contact;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231109_190937_c_student::Migration),
            Box::new(m20231113_170500_c_teacher::Migration),
            Box::new(m20231116_165911_c_parents::Migration),
            Box::new(m20231116_171406_c_pickups::Migration),
            Box::new(m20231116_214011_c_scans::Migration),
            Box::new(m20231116_234108_c_subject::Migration),
            Box::new(m20231117_132237_c_level::Migration),
            Box::new(m20231118_095513_c_contact::Migration),
        ]
    }
}
