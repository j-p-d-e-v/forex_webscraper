pub use sea_orm_migration::prelude::*;

mod m20240309_235049_forex_factory_news;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240309_235049_forex_factory_news::Migration),
        ]
    }
}
