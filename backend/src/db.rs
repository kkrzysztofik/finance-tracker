use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    info!("Connected to PostgreSQL");
    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    let migrations = [
        include_str!("../migrations/001_initial_schema.sql"),
        include_str!("../migrations/002_seed_categories.sql"),
    ];

    for (i, migration) in migrations.iter().enumerate() {
        info!("Running migration {}", i + 1);
        sqlx::raw_sql(migration).execute(pool).await?;
    }

    info!("All migrations completed");
    Ok(())
}
