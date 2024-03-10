use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(ForexFactoryNews::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ForexFactoryNews::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ForexFactoryNews::Year).integer().not_null())
                    .col(ColumnDef::new(ForexFactoryNews::Date).string().not_null())
                    .col(ColumnDef::new(ForexFactoryNews::DateLine).integer().not_null())
                    .col(ColumnDef::new(ForexFactoryNews::Actual).string().not_null())
                    .col(ColumnDef::new(ForexFactoryNews::Country).string().not_null())
                    .col(ColumnDef::new(ForexFactoryNews::Currency).string().not_null())
                    .col(ColumnDef::new(ForexFactoryNews::Forecast).string().not_null())
                    .col(ColumnDef::new(ForexFactoryNews::ImpactTitle).text().not_null())
                    .col(ColumnDef::new(ForexFactoryNews::Name).text().not_null())
                    .col(ColumnDef::new(ForexFactoryNews::Previous).string().not_null())
                    .col(ColumnDef::new(ForexFactoryNews::TimeLabel).string().not_null())
                    .col(ColumnDef::new(ForexFactoryNews::Url).text().not_null())
                    .col(ColumnDef::new(ForexFactoryNews::Timezone).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(ForexFactoryNews::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ForexFactoryNews {
    Table,
    Id,
    Year,
    Date,
    DateLine,
    Actual,
    Country,
    Currency,
    Forecast,
    ImpactTitle,
    Name,
    Previous,
    TimeLabel,
    Url,
    Timezone,
}
