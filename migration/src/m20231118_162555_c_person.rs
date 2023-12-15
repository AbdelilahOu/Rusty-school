use sea_orm_migration::prelude::*;

use super::{
    m20231109_190937_c_student::Student, m20231113_170500_c_teacher::Teacher,
    m20231116_165911_c_parents::Parent, m20231116_214011_c_scans::Scan,
    m20231118_095513_c_details::PersonDetails,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Person::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Person::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Person::PersonType).string().not_null())
                    .col(ColumnDef::new(Person::DetailsId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-details-person_id")
                            .from(Person::Table, Person::DetailsId)
                            .to(PersonDetails::Table, PersonDetails::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Scan::Table)
                    .add_column(ColumnDef::new(Scan::PersonId).uuid().not_null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-scan-person_id")
                            .from_tbl(Scan::Table)
                            .from_col(Scan::PersonId)
                            .to_tbl(Person::Table)
                            .to_col(Person::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Student::Table)
                    .add_column(ColumnDef::new(Student::PersonId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-student-person_id")
                            .from_tbl(Student::Table)
                            .from_col(Student::PersonId)
                            .to_tbl(Person::Table)
                            .to_col(Person::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Parent::Table)
                    .add_column(ColumnDef::new(Parent::PersonId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-parent-person_id")
                            .from_tbl(Parent::Table)
                            .from_col(Parent::PersonId)
                            .to_tbl(Person::Table)
                            .to_col(Person::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Teacher::Table)
                    .add_column(ColumnDef::new(Teacher::PersonId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-teacher-person_id")
                            .from_tbl(Teacher::Table)
                            .from_col(Teacher::PersonId)
                            .to_tbl(Person::Table)
                            .to_col(Person::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk-scan-person_id")
                    .table(Scan::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Scan::Table)
                    .drop_column(Scan::PersonId)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk-student-person_id")
                    .table(Student::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Student::Table)
                    .drop_column(Student::PersonId)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk-parent-person_id")
                    .table(Parent::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Parent::Table)
                    .drop_column(Parent::PersonId)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk-teacher-person_id")
                    .table(Teacher::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Teacher::Table)
                    .drop_column(Teacher::PersonId)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Person::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Person {
    #[sea_orm(iden = "persons")]
    Table,
    Id,
    #[sea_orm(iden = "details_id")]
    DetailsId,
    #[sea_orm(iden = "person_type")]
    PersonType,
}
