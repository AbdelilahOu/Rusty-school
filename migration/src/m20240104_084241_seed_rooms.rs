use sea_orm_migration::prelude::*;

use crate::{m20231223_093909_c_rooms::Room, utils::generate_random_room};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for _ in 0..=50 {
            let room = generate_random_room();
            let insert = Query::insert()
                .into_table(Room::Table)
                .columns([Room::Name, Room::Description])
                .values_panic([room.room_name.into(), room.room_description.into()])
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_query = Query::delete().from_table(Room::Table).to_owned();
        manager.exec_stmt(delete_query).await?;
        Ok(())
    }
}
