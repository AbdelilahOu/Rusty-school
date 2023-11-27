use sea_orm_migration::prelude::*;

use super::{
    m20231109_190937_c_student::Student, m20231113_170500_c_teacher::Teacher,
    m20231116_165911_c_parents::Parent, m20231116_214011_c_scans::Scan,
    m20231118_095513_c_contact::ContactInformation,
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
                    .col(ColumnDef::new(Person::PersonType).string().not_null())
                    .col(ColumnDef::new(Person::ContactId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-contact-person_id")
                            .from(Person::Table, Person::ContactId)
                            .to(ContactInformation::Table, ContactInformation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await;

        if let Err(e) = person_res {
            return Err(e);
        }

        let person_scan = manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Scan::Table)
                    .add_column(ColumnDef::new(Scan::PersonId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_scan_person")
                            .from_tbl(Scan::Table)
                            .from_col(Scan::PersonId)
                            .to_tbl(Person::Table)
                            .to_col(Person::Id),
                    )
                    .to_owned(),
            )
            .await;

        if let Err(e) = person_scan {
            return Err(e);
        }

        let person_student = manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Student::Table)
                    .add_column(ColumnDef::new(Student::PersonId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_student_person")
                            .from_tbl(Student::Table)
                            .from_col(Student::PersonId)
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
                    .table(Parent::Table)
                    .add_column(ColumnDef::new(Parent::PersonId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_parent_person")
                            .from_tbl(Parent::Table)
                            .from_col(Parent::PersonId)
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
                    .table(Teacher::Table)
                    .add_column(ColumnDef::new(Teacher::PersonId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_teacher_person")
                            .from_tbl(Teacher::Table)
                            .from_col(Teacher::PersonId)
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
        let drop_fk_scan = manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_scan_person")
                    .table(Scan::Table)
                    .to_owned(),
            )
            .await;

        if let Err(e) = drop_fk_scan {
            return Err(e);
        }

        let drop_scan_person_id = manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Scan::Table)
                    .drop_column(Scan::PersonId)
                    .to_owned(),
            )
            .await;

        if let Err(e) = drop_scan_person_id {
            return Err(e);
        }

        let drop_fk_student = manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_student_person")
                    .table(Student::Table)
                    .to_owned(),
            )
            .await;

        if let Err(e) = drop_fk_student {
            return Err(e);
        }

        let drop_student_person_id = manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Student::Table)
                    .drop_column(Student::PersonId)
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
                    .table(Parent::Table)
                    .to_owned(),
            )
            .await;

        if let Err(e) = drop_fk_parent {
            return Err(e);
        }

        let drop_parent_person_id = manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Parent::Table)
                    .drop_column(Parent::PersonId)
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
                    .table(Teacher::Table)
                    .to_owned(),
            )
            .await;

        if let Err(e) = drop_fk_teacher {
            return Err(e);
        }

        let drop_teacher_person_id = manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Teacher::Table)
                    .drop_column(Teacher::PersonId)
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
pub enum Person {
    #[sea_orm(iden = "persons")]
    Table,
    Id,
    #[sea_orm(iden = "contact_id")]
    ContactId,
    #[sea_orm(iden = "person_type")]
    PersonType,
}
