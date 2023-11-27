use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let countries_res = manager
            .create_table(
                Table::create()
                    .table(Country::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Country::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Country::CName).string().not_null())
                    .col(ColumnDef::new(Country::Cinitials).string())
                    .to_owned(),
            )
            .await;

        if let Err(e) = countries_res {
            return Err(e);
        }

        let states_res = manager
            .create_table(
                Table::create()
                    .table(State::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(State::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(State::SName).string().not_null())
                    .col(ColumnDef::new(State::Sinitials).string())
                    .col(ColumnDef::new(State::SCode).integer())
                    .col(ColumnDef::new(State::CountyId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-states-country_id")
                            .from(State::Table, State::CountyId)
                            .to(Country::Table, Country::Id)
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
                    .table(City::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(City::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(City::CiName).string().not_null())
                    .col(ColumnDef::new(City::StateId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-cities-state_id")
                            .from(City::Table, City::StateId)
                            .to(State::Table, State::Id)
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
                    .table(District::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(District::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(District::DName).string().not_null())
                    .col(ColumnDef::new(District::CityId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-districts-city_id")
                            .from(District::Table, District::CityId)
                            .to(City::Table, City::Id)
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
                    .table(Street::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Street::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Street::StName).string().not_null())
                    .col(ColumnDef::new(Street::ZipCode).integer())
                    .col(ColumnDef::new(Street::StreetType).string())
                    .col(ColumnDef::new(Street::DistrictId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-streets-district_id")
                            .from(Street::Table, Street::DistrictId)
                            .to(District::Table, District::Id)
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
                    .table(ContactInformation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ContactInformation::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ContactInformation::Phone).string())
                    .col(
                        ColumnDef::new(ContactInformation::Email)
                            .string()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(ContactInformation::Country).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-contacts-country_id")
                            .from(ContactInformation::Table, ContactInformation::Country)
                            .to(Country::Table, Country::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(ContactInformation::State).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-contacts-state_id")
                            .from(ContactInformation::Table, ContactInformation::State)
                            .to(State::Table, State::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(ContactInformation::City).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-contacts-city_id")
                            .from(ContactInformation::Table, ContactInformation::City)
                            .to(City::Table, City::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(ContactInformation::District).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-contacts-district_id")
                            .from(ContactInformation::Table, ContactInformation::District)
                            .to(District::Table, District::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(ContactInformation::Street).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-contacts-street_id")
                            .from(ContactInformation::Table, ContactInformation::Street)
                            .to(Street::Table, Street::Id)
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
            .drop_table(Table::drop().table(ContactInformation::Table).to_owned())
            .await;

        if let Err(e) = contacts_drop {
            return Err(e);
        }

        let streets_drop = manager
            .drop_table(Table::drop().table(Street::Table).to_owned())
            .await;

        if let Err(e) = streets_drop {
            return Err(e);
        }

        let districts_table = manager
            .drop_table(Table::drop().table(District::Table).to_owned())
            .await;

        if let Err(e) = districts_table {
            return Err(e);
        }

        let cities_table = manager
            .drop_table(Table::drop().table(City::Table).to_owned())
            .await;

        if let Err(e) = cities_table {
            return Err(e);
        }

        let states_drop = manager
            .drop_table(Table::drop().table(State::Table).to_owned())
            .await;

        if let Err(e) = states_drop {
            return Err(e);
        }

        let coutries_drop = manager
            .drop_table(Table::drop().table(Country::Table).to_owned())
            .await;

        if let Err(e) = coutries_drop {
            return Err(e);
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum ContactInformation {
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
enum Country {
    #[sea_orm(iden = "countries")]
    Table,
    Id,
    #[sea_orm(iden = "country_name")]
    CName,
    #[sea_orm(iden = "country_initials")]
    Cinitials,
}

#[derive(DeriveIden)]
enum State {
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
enum City {
    #[sea_orm(iden = "cities")]
    Table,
    Id,
    #[sea_orm(iden = "city_name")]
    CiName,
    #[sea_orm(iden = "state_id")]
    StateId,
}

#[derive(DeriveIden)]
enum District {
    #[sea_orm(iden = "districts")]
    Table,
    Id,
    #[sea_orm(iden = "district_name")]
    DName,
    #[sea_orm(iden = "city_id")]
    CityId,
}

#[derive(DeriveIden)]
enum Street {
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
