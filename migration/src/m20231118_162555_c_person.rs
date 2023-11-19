use sea_orm_migration::prelude::*;

use super::{
    m20231109_190937_c_student::Students, m20231113_170500_c_teacher::Teachers,
    m20231116_165911_c_parents::Parents, m20231118_095513_c_contact::ContactInformations,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let person_res = manager
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
                    .col(ColumnDef::new(Person::ContactId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-contact-person_id")
                            .from(Person::Table, Person::ContactId)
                            .to(ContactInformations::Table, ContactInformations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await;

        if let Err(e) = person_res {
            return Err(e);
        }

        let person_student = manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Students::Table)
                    .add_column(ColumnDef::new(Students::PersonId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_student_person")
                            .from_tbl(Students::Table)
                            .from_col(Students::PersonId)
                            .to_tbl(Person::Table)
                            .to_col(Person::Id),
                    )
                    .to_owned(),
            )
            .await;

        if let Err(e) = person_student {
            return Err(e);
        }

        let person_parent = manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Parents::Table)
                    .add_column(ColumnDef::new(Parents::PersonId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_parent_person")
                            .from_tbl(Parents::Table)
                            .from_col(Parents::PersonId)
                            .to_tbl(Person::Table)
                            .to_col(Person::Id),
                    )
                    .to_owned(),
            )
            .await;

        if let Err(e) = person_parent {
            return Err(e);
        }

        let person_teacher = manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Teachers::Table)
                    .add_column(ColumnDef::new(Teachers::PersonId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_teacher_person")
                            .from_tbl(Teachers::Table)
                            .from_col(Teachers::PersonId)
                            .to_tbl(Person::Table)
                            .to_col(Person::Id),
                    )
                    .to_owned(),
            )
            .await;

        if let Err(e) = person_teacher {
            return Err(e);
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let drop_fk_student = manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_student_person")
                    .table(Students::Table)
                    .to_owned(),
            )
            .await;

        if let Err(e) = drop_fk_student {
            return Err(e);
        }

        let drop_student_person_id = manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Students::Table)
                    .drop_column(Students::PersonId)
                    .to_owned(),
            )
            .await;

        if let Err(e) = drop_student_person_id {
            return Err(e);
        }

        let drop_fk_parent = manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_parent_person")
                    .table(Parents::Table)
                    .to_owned(),
            )
            .await;

        if let Err(e) = drop_fk_parent {
            return Err(e);
        }

        let drop_parent_person_id = manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Parents::Table)
                    .drop_column(Parents::PersonId)
                    .to_owned(),
            )
            .await;

        if let Err(e) = drop_parent_person_id {
            return Err(e);
        }

        let drop_fk_teacher = manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_teacher_person")
                    .table(Teachers::Table)
                    .to_owned(),
            )
            .await;

        if let Err(e) = drop_fk_teacher {
            return Err(e);
        }

        let drop_teacher_person_id = manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Teachers::Table)
                    .drop_column(Teachers::PersonId)
                    .to_owned(),
            )
            .await;

        if let Err(e) = drop_teacher_person_id {
            return Err(e);
        }

        let drop_table = manager
            .drop_table(Table::drop().table(Person::Table).to_owned())
            .await;

        if let Err(e) = drop_table {
            return Err(e);
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Person {
    #[sea_orm(iden = "persons")]
    Table,
    Id,
    #[sea_orm(iden = "contact_id")]
    ContactId,
}
