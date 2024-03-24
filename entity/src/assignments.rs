//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "assignments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub due_date: DateTime,
    pub submission_type: String,
    pub gradin_rubric_id: Option<Uuid>,
    pub file: Option<String>,
    pub teacher_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::grading_rubrics::Entity",
        from = "Column::GradinRubricId",
        to = "super::grading_rubrics::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    GradingRubrics,
    #[sea_orm(
        belongs_to = "super::teachers::Entity",
        from = "Column::GradinRubricId",
        to = "super::teachers::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Teachers,
}

impl Related<super::grading_rubrics::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GradingRubrics.def()
    }
}

impl Related<super::teachers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teachers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
