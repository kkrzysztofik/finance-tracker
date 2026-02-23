use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::info;

pub async fn create_pool(database_url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(10);

    let db = Database::connect(opt).await?;
    info!("Connected to PostgreSQL");
    Ok(db)
}

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    use sea_orm::ConnectionTrait;

    let migrations = [
        include_str!("../migrations/001_initial_schema.sql"),
        include_str!("../migrations/002_seed_categories.sql"),
    ];

    for (i, migration) in migrations.iter().enumerate() {
        info!("Running migration {}", i + 1);
        db.execute_unprepared(migration).await?;
    }

    info!("All migrations completed");
    Ok(())
}
