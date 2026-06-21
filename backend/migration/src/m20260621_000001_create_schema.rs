use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Accounts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Accounts::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Accounts::Name).text().not_null())
                    .col(
                        ColumnDef::new(Accounts::Currency)
                            .text()
                            .not_null()
                            .default("PLN"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-accounts-name-unique")
                    .table(Accounts::Table)
                    .col(Accounts::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Categories::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Categories::Name).text().not_null())
                    .col(ColumnDef::new(Categories::NamePl).text())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-categories-name-unique")
                    .table(Categories::Table)
                    .col(Categories::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Transactions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transactions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Transactions::Hash).text().not_null())
                    .col(ColumnDef::new(Transactions::AccountId).integer().not_null())
                    .col(
                        ColumnDef::new(Transactions::TransactionDate)
                            .date()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Transactions::BookingDate).date())
                    .col(ColumnDef::new(Transactions::Counterparty).text())
                    .col(ColumnDef::new(Transactions::Description).text().not_null())
                    .col(
                        ColumnDef::new(Transactions::Amount)
                            .decimal_len(12, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Transactions::Currency)
                            .text()
                            .not_null()
                            .default("PLN"),
                    )
                    .col(ColumnDef::new(Transactions::CategoryId).integer())
                    .col(ColumnDef::new(Transactions::CategorySource).text())
                    .col(ColumnDef::new(Transactions::BankCategory).text())
                    .col(ColumnDef::new(Transactions::BankReference).text())
                    .col(ColumnDef::new(Transactions::BankType).text())
                    .col(
                        ColumnDef::new(Transactions::State)
                            .text()
                            .default("completed"),
                    )
                    .col(ColumnDef::new(Transactions::RawData).json_binary())
                    .col(
                        ColumnDef::new(Transactions::ImportedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-transactions-account_id")
                            .from(Transactions::Table, Transactions::AccountId)
                            .to(Accounts::Table, Accounts::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-transactions-category_id")
                            .from(Transactions::Table, Transactions::CategoryId)
                            .to(Categories::Table, Categories::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-transactions-hash-unique")
                    .table(Transactions::Table)
                    .col(Transactions::Hash)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ImportLogs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ImportLogs::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ImportLogs::Filename).text().not_null())
                    .col(ColumnDef::new(ImportLogs::AccountId).integer().not_null())
                    .col(ColumnDef::new(ImportLogs::TotalRows).integer().not_null())
                    .col(ColumnDef::new(ImportLogs::Imported).integer().not_null())
                    .col(ColumnDef::new(ImportLogs::Skipped).integer().not_null())
                    .col(
                        ColumnDef::new(ImportLogs::ImportedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-import_logs-account_id")
                            .from(ImportLogs::Table, ImportLogs::AccountId)
                            .to(Accounts::Table, Accounts::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "INSERT INTO accounts (name, currency) VALUES
                    ('alior', 'PLN'),
                    ('pekao', 'PLN'),
                    ('revolut', 'PLN')
                 ON CONFLICT (name) DO NOTHING;",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ImportLogs::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(Transactions::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(Categories::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(Accounts::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Accounts {
    Table,
    Id,
    Name,
    Currency,
}

#[derive(DeriveIden)]
enum Categories {
    Table,
    Id,
    Name,
    NamePl,
}

#[derive(DeriveIden)]
enum Transactions {
    Table,
    Id,
    Hash,
    AccountId,
    TransactionDate,
    BookingDate,
    Counterparty,
    Description,
    Amount,
    Currency,
    CategoryId,
    CategorySource,
    BankCategory,
    BankReference,
    BankType,
    State,
    RawData,
    ImportedAt,
}

#[derive(DeriveIden)]
enum ImportLogs {
    Table,
    Id,
    Filename,
    AccountId,
    TotalRows,
    Imported,
    Skipped,
    ImportedAt,
}
