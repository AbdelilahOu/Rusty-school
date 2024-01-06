pub use sea_orm_migration::prelude::*;

mod m20231109_190937_c_students;
mod m20231113_170500_c_teachers;
mod m20231116_165911_c_parents;
mod m20231116_171406_c_pickups;
mod m20231116_214011_c_scans;
mod m20231118_095513_c_details;
mod m20231118_162555_c_persons;
mod m20231127_123039_c_users;
mod m20231211_172237_c_levels;
mod m20231215_142739_c_subjects;
mod m20231222_155651_c_groups;
mod m20231223_093909_c_rooms;
mod m20231223_094755_c_classes;
mod m20231229_133921_seed_levels;
mod m20231229_182506_seed_students;
mod m20231229_182513_seed_parents;
mod m20231229_182522_seed_teachers;
mod m20231229_183159_seed_pickups;
mod m20231229_194513_seed_details;
mod m20231229_195116_seed_persons;
mod m20240101_135233_seed_scans;
mod m20240104_081431_seed_subjects;
mod m20240104_083346_seed_groups;
mod m20240104_102705_seed_student_group;

//
mod utils;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231109_190937_c_students::Migration),
            Box::new(m20231113_170500_c_teachers::Migration),
            Box::new(m20231116_165911_c_parents::Migration),
            Box::new(m20231116_171406_c_pickups::Migration),
            Box::new(m20231116_214011_c_scans::Migration),
            Box::new(m20231118_095513_c_details::Migration),
            Box::new(m20231118_162555_c_persons::Migration),
            Box::new(m20231127_123039_c_users::Migration),
            Box::new(m20231211_172237_c_levels::Migration),
            Box::new(m20231215_142739_c_subjects::Migration),
            Box::new(m20231222_155651_c_groups::Migration),
            Box::new(m20231223_093909_c_rooms::Migration),
            Box::new(m20231223_094755_c_classes::Migration),
            Box::new(m20231229_133921_seed_levels::Migration),
            Box::new(m20231229_182506_seed_students::Migration),
            Box::new(m20231229_182513_seed_parents::Migration),
            Box::new(m20231229_182522_seed_teachers::Migration),
            Box::new(m20231229_183159_seed_pickups::Migration),
            Box::new(m20231229_194513_seed_details::Migration),
            Box::new(m20231229_195116_seed_persons::Migration),
            Box::new(m20240101_135233_seed_scans::Migration),
            Box::new(m20240104_081431_seed_subjects::Migration),
            Box::new(m20240104_083346_seed_groups::Migration),
            Box::new(m20240104_102705_seed_student_group::Migration),
        ]
    }
}
