//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "streets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub street_name: String,
    pub zip_code: Option<String>,
    pub street_type: Option<String>,
    pub district_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::districts::Entity",
        from = "Column::DistrictId",
        to = "super::districts::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Districts,
    #[sea_orm(has_many = "super::person_details::Entity")]
    PersonDetails,
}

impl Related<super::districts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Districts.def()
    }
}

impl Related<super::person_details::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PersonDetails.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
