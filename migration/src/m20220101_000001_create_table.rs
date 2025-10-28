use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Users::Email).string_len(255).not_null().unique_key())
                    .col(ColumnDef::new(Users::Password).string_len(255).not_null())
                    .col(ColumnDef::new(Users::FirstName).string_len(255))
                    .col(ColumnDef::new(Users::LastName).string_len(255))
                    .col(ColumnDef::new(Users::AccountType).string_len(32).not_null())
                    .col(ColumnDef::new(Users::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Users::CreatedAt).timestamp())
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        // Create hosts table (user_id as PK and FK -> users.id)
        manager
            .create_table(
                Table::create()
                    .table(Hosts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Hosts::UserId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Hosts::OrganizationName).string_len(255))
                    .col(ColumnDef::new(Hosts::EventsHostedCount).integer().not_null().default(0))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_hosts_user")
                            .from(Hosts::Table, Hosts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create attendees table (user_id as PK and FK -> users.id)
        manager
            .create_table(
                Table::create()
                    .table(Attendees::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Attendees::UserId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_attendees_user")
                            .from(Attendees::Table, Attendees::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create events table
        manager
            .create_table(
                Table::create()
                    .table(Events::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Events::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Events::Title).string_len(255).not_null())
                    .col(ColumnDef::new(Events::Description).text().not_null())
                    .col(ColumnDef::new(Events::Location).string_len(255).not_null())
                    .col(ColumnDef::new(Events::EventType).string_len(32).not_null())
                    .col(ColumnDef::new(Events::Category).string_len(32).not_null())
                    .col(ColumnDef::new(Events::Status).string_len(32).not_null())
                    .col(ColumnDef::new(Events::Visibility).string_len(32).not_null())
                    .col(ColumnDef::new(Events::HostId).integer().not_null())
                    .col(ColumnDef::new(Events::StartTime).timestamp().not_null())
                    .col(ColumnDef::new(Events::EndTime).timestamp().not_null())
                    .col(ColumnDef::new(Events::CreatedAt).timestamp())
                    .col(ColumnDef::new(Events::UpdatedAt).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_events_host")
                            .from(Events::Table, Events::HostId)
                            .to(Hosts::Table, Hosts::UserId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create skills table
        manager
            .create_table(
                Table::create()
                    .table(Skills::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Skills::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Skills::UserId).integer().not_null())
                    .col(ColumnDef::new(Skills::Name).string_len(128).not_null())
                    .col(ColumnDef::new(Skills::CreatedAt).timestamp())
                    .col(ColumnDef::new(Skills::UpdatedAt).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_skills_user")
                            .from(Skills::Table, Skills::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create attendance table
        manager
            .create_table(
                Table::create()
                    .table(Attendance::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Attendance::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Attendance::EventId).integer().not_null())
                    .col(ColumnDef::new(Attendance::AttendeeId).integer().not_null())
                    .col(ColumnDef::new(Attendance::Status).string_len(32).not_null())
                    .col(ColumnDef::new(Attendance::CreatedAt).timestamp())
                    .col(ColumnDef::new(Attendance::UpdatedAt).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_attendance_event")
                            .from(Attendance::Table, Attendance::EventId)
                            .to(Events::Table, Events::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_attendance_attendee")
                            .from(Attendance::Table, Attendance::AttendeeId)
                            .to(Attendees::Table, Attendees::UserId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Attendance::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Skills::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Events::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Attendees::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Hosts::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Email,
    Password,
    FirstName,
    LastName,
    AccountType,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Hosts {
    Table,
    UserId,
    OrganizationName,
    EventsHostedCount,
}

#[derive(DeriveIden)]
enum Attendees {
    Table,
    UserId,
}

#[derive(DeriveIden)]
enum Events {
    Table,
    Id,
    Title,
    Description,
    Location,
    EventType,
    Category,
    Status,
    Visibility,
    HostId,
    StartTime,
    EndTime,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Skills {
    Table,
    Id,
    UserId,
    Name,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Attendance {
    Table,
    Id,
    EventId,
    AttendeeId,
    Status,
    CreatedAt,
    UpdatedAt,
}

