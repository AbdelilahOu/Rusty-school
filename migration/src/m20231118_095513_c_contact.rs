use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let countries_res = manager
            .create_table(
                Table::create()
                    .table(Countries::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Countries::Id).uuid().primary_key())
                    .col(ColumnDef::new(Countries::CName).string().not_null())
                    .col(ColumnDef::new(Countries::Cinitials).string())
                    .to_owned(),
            )
            .await;

        if let Err(e) = countries_res {
            return Err(e);
        }

        let states_res = manager
            .create_table(
                Table::create()
                    .table(States::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(States::Id).uuid().primary_key())
                    .col(ColumnDef::new(States::SName).string().not_null())
                    .col(ColumnDef::new(States::Sinitials).string())
                    .col(ColumnDef::new(States::SCode).integer())
                    .col(ColumnDef::new(States::CountyId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-states-country_id")
                            .from(States::Table, States::CountyId)
                            .to(Countries::Table, Countries::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await;

        if let Err(e) = states_res {
            return Err(e);
        }

        let cities_res = manager
            .create_table(
                Table::create()
                    .table(Cities::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Cities::Id).uuid().primary_key())
                    .col(ColumnDef::new(Cities::CiName).string().not_null())
                    .col(ColumnDef::new(Cities::StateId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-cities-state_id")
                            .from(Cities::Table, Cities::StateId)
                            .to(States::Table, States::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await;

        if let Err(e) = cities_res {
            return Err(e);
        }

        let districts_res = manager
            .create_table(
                Table::create()
                    .table(Districts::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Districts::Id).uuid().primary_key())
                    .col(ColumnDef::new(Districts::DName).string().not_null())
                    .col(ColumnDef::new(Districts::CityId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-districts-city_id")
                            .from(Districts::Table, Districts::CityId)
                            .to(Cities::Table, Cities::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await;

        if let Err(e) = districts_res {
            return Err(e);
        }

        let streets_res = manager
            .create_table(
                Table::create()
                    .table(Streets::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Streets::Id).uuid().primary_key())
                    .col(ColumnDef::new(Streets::StName).string().not_null())
                    .col(ColumnDef::new(Streets::ZipCode).integer())
                    .col(ColumnDef::new(Streets::StreetType).string())
                    .col(ColumnDef::new(Streets::DistrictId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-streets-district_id")
                            .from(Streets::Table, Streets::DistrictId)
                            .to(Districts::Table, Districts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await;

        if let Err(e) = streets_res {
            return Err(e);
        }

        let contact_res = manager
            .create_table(
                Table::create()
                    .table(ContactInformations::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ContactInformations::Id).uuid().primary_key())
                    .col(ColumnDef::new(ContactInformations::Phone).string())
                    .col(
                        ColumnDef::new(ContactInformations::Email)
                            .string()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(ContactInformations::Country).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-contacts-country_id")
                            .from(ContactInformations::Table, ContactInformations::Country)
                            .to(Countries::Table, Countries::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(ContactInformations::State).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-contacts-state_id")
                            .from(ContactInformations::Table, ContactInformations::State)
                            .to(States::Table, States::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(ContactInformations::City).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-contacts-city_id")
                            .from(ContactInformations::Table, ContactInformations::City)
                            .to(Cities::Table, Cities::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(ContactInformations::District).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-contacts-district_id")
                            .from(ContactInformations::Table, ContactInformations::District)
                            .to(Districts::Table, Districts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(ContactInformations::Street).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-contacts-street_id")
                            .from(ContactInformations::Table, ContactInformations::Street)
                            .to(Streets::Table, Streets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await;

        if let Err(e) = contact_res {
            return Err(e);
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let contacts_drop = manager
            .drop_table(Table::drop().table(ContactInformations::Table).to_owned())
            .await;

        if let Err(e) = contacts_drop {
            return Err(e);
        }

        let streets_drop = manager
            .drop_table(Table::drop().table(Streets::Table).to_owned())
            .await;

        if let Err(e) = streets_drop {
            return Err(e);
        }

        let districts_table = manager
            .drop_table(Table::drop().table(Districts::Table).to_owned())
            .await;

        if let Err(e) = districts_table {
            return Err(e);
        }

        let cities_table = manager
            .drop_table(Table::drop().table(Cities::Table).to_owned())
            .await;

        if let Err(e) = cities_table {
            return Err(e);
        }

        let states_drop = manager
            .drop_table(Table::drop().table(States::Table).to_owned())
            .await;

        if let Err(e) = states_drop {
            return Err(e);
        }

        let coutries_drop = manager
            .drop_table(Table::drop().table(Countries::Table).to_owned())
            .await;

        if let Err(e) = coutries_drop {
            return Err(e);
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum ContactInformations {
    #[sea_orm(iden = "contacts_informations")]
    Table,
    Id,
    #[sea_orm(iden = "phone_number")]
    Phone,
    Email,
    #[sea_orm(iden = "country_id")]
    Country,
    #[sea_orm(iden = "state_id")]
    State,
    #[sea_orm(iden = "city_id")]
    City,
    #[sea_orm(iden = "district_id")]
    District,
    #[sea_orm(iden = "street_id")]
    Street,
}

#[derive(DeriveIden)]
enum Countries {
    #[sea_orm(iden = "countries")]
    Table,
    Id,
    #[sea_orm(iden = "country_name")]
    CName,
    #[sea_orm(iden = "country_initials")]
    Cinitials,
}

#[derive(DeriveIden)]
enum States {
    #[sea_orm(iden = "states")]
    Table,
    Id,
    #[sea_orm(iden = "state_name")]
    SName,
    #[sea_orm(iden = "state_initials")]
    Sinitials,
    #[sea_orm(iden = "state_code")]
    SCode,
    #[sea_orm(iden = "country_id")]
    CountyId,
}

#[derive(DeriveIden)]
enum Cities {
    #[sea_orm(iden = "cities")]
    Table,
    Id,
    #[sea_orm(iden = "city_name")]
    CiName,
    #[sea_orm(iden = "state_id")]
    StateId,
}

#[derive(DeriveIden)]
enum Districts {
    #[sea_orm(iden = "districts")]
    Table,
    Id,
    #[sea_orm(iden = "district_name")]
    DName,
    #[sea_orm(iden = "city_id")]
    CityId,
}

#[derive(DeriveIden)]
enum Streets {
    #[sea_orm(iden = "streets")]
    Table,
    Id,
    #[sea_orm(iden = "street_name")]
    StName,
    #[sea_orm(iden = "zip_code")]
    ZipCode,
    #[sea_orm(iden = "street_type")]
    StreetType,
    #[sea_orm(iden = "district_id")]
    DistrictId,
}

// streettype : street || avenue || easement
