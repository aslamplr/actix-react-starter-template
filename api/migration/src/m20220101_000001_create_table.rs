use sea_orm_migration::prelude::*;

use entity::{cake, fruit};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(cake::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(cake::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(cake::Column::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                sea_query::Table::create()
                    .table(fruit::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(fruit::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(fruit::Column::Name).string().not_null())
                    .col(ColumnDef::new(fruit::Column::CakeId).integer().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .name("FK_fruit_cake")
                    .from(fruit::Entity, fruit::Column::CakeId)
                    .to(cake::Entity, cake::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("FK_fruit_cake")
                    .table(fruit::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(fruit::Entity)
                    .table(cake::Entity)
                    .to_owned(),
            )
            .await
    }
}
