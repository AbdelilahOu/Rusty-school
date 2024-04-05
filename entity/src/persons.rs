//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use super::sea_orm_active_enums::PersonTypeEnum;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "persons")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub person_type: PersonTypeEnum,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::parents::Entity")]
    Parents,
    #[sea_orm(has_many = "super::scans::Entity")]
    Scans,
    #[sea_orm(has_many = "super::students::Entity")]
    Students,
    #[sea_orm(has_many = "super::teachers::Entity")]
    Teachers,
    #[sea_orm(has_many = "super::users::Entity")]
    Users,
}

impl Related<super::parents::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Parents.def()
    }
}

impl Related<super::scans::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Scans.def()
    }
}

impl Related<super::students::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Students.def()
    }
}

impl Related<super::teachers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teachers.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
