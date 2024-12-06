pub use sea_orm_migration::prelude::*;

mod m20241201_143409_create_app_reservation_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20241201_143409_create_app_reservation_table::Migration,
        )]
    }
}
