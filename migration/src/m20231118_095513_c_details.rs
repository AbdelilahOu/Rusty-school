use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
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
            .await?;

        manager
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
                    .col(ColumnDef::new(State::CountyId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_states_country_id")
                            .from(State::Table, State::CountyId)
                            .to(Country::Table, Country::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
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
                            .name("fk_cities_state_id")
                            .from(City::Table, City::StateId)
                            .to(State::Table, State::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
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
                            .name("fk_districts_city_id")
                            .from(District::Table, District::CityId)
                            .to(City::Table, City::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
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
                    .col(ColumnDef::new(Street::ZipCode).string())
                    .col(ColumnDef::new(Street::StreetType).string())
                    .col(ColumnDef::new(Street::DistrictId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_streets_district_id")
                            .from(Street::Table, Street::DistrictId)
                            .to(District::Table, District::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PersonDetails::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PersonDetails::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PersonDetails::Phone).string())
                    .col(ColumnDef::new(PersonDetails::Email).string().unique_key())
                    .col(ColumnDef::new(PersonDetails::Country).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_contacts_country_id")
                            .from(PersonDetails::Table, PersonDetails::Country)
                            .to(Country::Table, Country::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(PersonDetails::State).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_contacts_state_id")
                            .from(PersonDetails::Table, PersonDetails::State)
                            .to(State::Table, State::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(PersonDetails::City).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_contacts_city_id")
                            .from(PersonDetails::Table, PersonDetails::City)
                            .to(City::Table, City::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(PersonDetails::District).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_contacts_district_id")
                            .from(PersonDetails::Table, PersonDetails::District)
                            .to(District::Table, District::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(PersonDetails::Street).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_contacts_street_id")
                            .from(PersonDetails::Table, PersonDetails::Street)
                            .to(Street::Table, Street::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PersonDetails::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Street::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(District::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(City::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(State::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Country::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum PersonDetails {
    #[sea_orm(iden = "person_details")]
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
pub enum Country {
    #[sea_orm(iden = "countries")]
    Table,
    Id,
    #[sea_orm(iden = "country_name")]
    CName,
    #[sea_orm(iden = "country_initials")]
    Cinitials,
}

#[derive(DeriveIden)]
pub enum State {
    #[sea_orm(iden = "states")]
    Table,
    Id,
    #[sea_orm(iden = "state_name")]
    SName,
    #[sea_orm(iden = "state_initials")]
    Sinitials,
    #[sea_orm(iden = "country_id")]
    CountyId,
}

#[derive(DeriveIden)]
pub enum City {
    #[sea_orm(iden = "cities")]
    Table,
    Id,
    #[sea_orm(iden = "city_name")]
    CiName,
    #[sea_orm(iden = "state_id")]
    StateId,
}

#[derive(DeriveIden)]
pub enum District {
    #[sea_orm(iden = "districts")]
    Table,
    Id,
    #[sea_orm(iden = "district_name")]
    DName,
    #[sea_orm(iden = "city_id")]
    CityId,
}

#[derive(DeriveIden)]
pub enum Street {
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
