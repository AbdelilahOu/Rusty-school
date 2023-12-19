use sea_orm::{prelude::Uuid, Statement};
use sea_orm_migration::prelude::*;

use crate::{
    m20231118_095513_c_details::{City, Country, District, PersonDetails, State, Street},
    utils::{
        generate_random_city, generate_random_country, generate_random_details,
        generate_random_district, generate_random_state, generate_random_street,
    },
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        for _ in 0..5 {
            let rand_country = generate_random_country();
            let (sql, values) = Query::insert()
                .into_table(Country::Table)
                .columns([Country::CName, Country::Cinitials])
                .values_panic([
                    rand_country.country_name.into(),
                    rand_country.country_initials.into(),
                ])
                .returning(ReturningClause::Columns(vec![ColumnRef::Column(
                    SeaRc::new(Country::Id),
                )]))
                .to_owned()
                .build(PostgresQueryBuilder);

            let statment =
                Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, sql, values);
            let row = db.query_one(statment).await?;
            let country_id = row.unwrap().try_get::<Uuid>("", "id").unwrap();
            for _ in 0..6 {
                let rand_state = generate_random_state(country_id);
                let (sql, values) = Query::insert()
                    .into_table(State::Table)
                    .columns([State::SName, State::Sinitials, State::CountyId])
                    .values_panic([
                        rand_state.state_name.into(),
                        rand_state.state_initials.into(),
                        rand_state.country_id.into(),
                    ])
                    .returning(ReturningClause::Columns(vec![ColumnRef::Column(
                        SeaRc::new(State::Id),
                    )]))
                    .to_owned()
                    .build(PostgresQueryBuilder);

                let statment =
                    Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, sql, values);

                let row = db.query_one(statment).await?;
                let state_id = row.unwrap().try_get::<Uuid>("", "id").unwrap();

                for _ in 0..5 {
                    let rand_city = generate_random_city(state_id);
                    let (sql, values) = Query::insert()
                        .into_table(City::Table)
                        .columns([City::CiName, City::StateId])
                        .values_panic([rand_city.city_name.into(), rand_city.state_id.into()])
                        .returning(ReturningClause::Columns(vec![ColumnRef::Column(
                            SeaRc::new(City::Id),
                        )]))
                        .to_owned()
                        .build(PostgresQueryBuilder);

                    let statment = Statement::from_sql_and_values(
                        sea_orm::DatabaseBackend::Postgres,
                        sql,
                        values,
                    );

                    let row = db.query_one(statment).await?;
                    let city_id = row.unwrap().try_get::<Uuid>("", "id").unwrap();
                    for _ in 0..5 {
                        let rand_district = generate_random_district(city_id);
                        let (sql, values) = Query::insert()
                            .into_table(District::Table)
                            .columns([District::DName, District::CityId])
                            .values_panic([
                                rand_district.district_name.into(),
                                rand_district.city_id.into(),
                            ])
                            .returning(ReturningClause::Columns(vec![ColumnRef::Column(
                                SeaRc::new(District::Id),
                            )]))
                            .to_owned()
                            .build(PostgresQueryBuilder);

                        let statment = Statement::from_sql_and_values(
                            sea_orm::DatabaseBackend::Postgres,
                            sql,
                            values,
                        );

                        let row = db.query_one(statment).await?;
                        let district_id = row.unwrap().try_get::<Uuid>("", "id").unwrap();
                        for _ in 0..5 {
                            let rand_street = generate_random_street(district_id);
                            let (sql, values) = Query::insert()
                                .into_table(Street::Table)
                                .columns([
                                    Street::StName,
                                    Street::ZipCode,
                                    Street::StreetType,
                                    Street::DistrictId,
                                ])
                                .values_panic([
                                    rand_street.street_name.into(),
                                    rand_street.zip_code.into(),
                                    rand_street.street_type.into(),
                                    rand_street.district_id.into(),
                                ])
                                .returning(ReturningClause::Columns(vec![ColumnRef::Column(
                                    SeaRc::new(Street::Id),
                                )]))
                                .to_owned()
                                .build(PostgresQueryBuilder);

                            let statment = Statement::from_sql_and_values(
                                sea_orm::DatabaseBackend::Postgres,
                                sql,
                                values,
                            );

                            let row = db.query_one(statment).await?;
                            let street_id = row.unwrap().try_get::<Uuid>("", "id").unwrap();
                            let details = generate_random_details();
                            let insert_details = Query::insert()
                                .into_table(PersonDetails::Table)
                                .columns([
                                    PersonDetails::Phone,
                                    PersonDetails::Email,
                                    PersonDetails::Country,
                                    PersonDetails::State,
                                    PersonDetails::City,
                                    PersonDetails::District,
                                    PersonDetails::Street,
                                ])
                                .values_panic([
                                    details.phone_number.into(),
                                    details.email.into(),
                                    country_id.into(),
                                    state_id.into(),
                                    city_id.into(),
                                    district_id.into(),
                                    street_id.into(),
                                ])
                                .to_owned();
                            manager.exec_stmt(insert_details).await?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_query = Query::delete().from_table(PersonDetails::Table).to_owned();
        manager.exec_stmt(delete_query).await?;
        let delete_query = Query::delete().from_table(Country::Table).to_owned();
        manager.exec_stmt(delete_query).await?;
        Ok(())
    }
}
