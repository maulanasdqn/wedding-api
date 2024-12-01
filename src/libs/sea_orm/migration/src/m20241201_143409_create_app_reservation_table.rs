use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AppReservation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AppReservation::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AppReservation::Fullname).string().not_null())
                    .col(
                        ColumnDef::new(AppReservation::Attendance)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AppReservation::Speech).string().not_null())
                    .col(ColumnDef::new(AppReservation::SpeechAudio).string().null())
                    .col(
                        ColumnDef::new(AppReservation::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AppReservation::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum AppReservation {
    Table,
    Id,
    Fullname,
    Attendance,
    Speech,
    SpeechAudio,
    CreatedAt,
}
