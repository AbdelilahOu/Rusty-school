//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "activities")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", nullable)]
    pub activity_title: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub activity_description: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub activity_type: Option<String>,
    pub time_table_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::time_table::Entity",
        from = "Column::TimeTableId",
        to = "super::time_table::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    TimeTable,
}

impl Related<super::time_table::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TimeTable.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}