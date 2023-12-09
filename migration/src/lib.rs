pub use sea_orm_migration::prelude::*;

mod school_related;
mod user_related;

use school_related::*;
use user_related::*;
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
            Box::new(m20231118_095513_c_details::Migration),
            Box::new(m20231118_162555_c_person::Migration),
            Box::new(m20231127_123039_c_user::Migration),
        ]
    }
}
