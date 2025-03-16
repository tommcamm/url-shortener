use sqlx::PgPool;

pub async fn run_migrations_if_needed(pool: &PgPool) -> anyhow::Result<()> {
    // Check if the table already exists
    let table_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT FROM information_schema.tables 
            WHERE table_name = 'urls'
        )",
    )
    .fetch_one(pool)
    .await?;

    if table_exists {
        tracing::info!("Tables already exist, skipping migrations");
        return Ok(());
    }

    tracing::info!("Running database migrations...");
    // Use transaction for safety
    let mut tx = pool.begin().await?;

    // Execute each statement separately
    for statement in include_str!("../../migrations/20240214_create_urls_table.sql")
        .split(';')
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        sqlx::query(statement).execute(&mut *tx).await?;
    }

    tx.commit().await?;
    tracing::info!("Migrations completed successfully");

    Ok(())
}
