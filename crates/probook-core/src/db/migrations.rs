use sqlx::PgPool;

/// Embedded migration files, sorted by version number.
/// Each tuple is (version, name, sql).
const MIGRATIONS: &[(&str, &str, &str)] = &[
    ("001", "initial_schema", include_str!("../../migrations/001_initial_schema.sql")),
    ("002", "delivery_notes", include_str!("../../migrations/002_delivery_notes.sql")),
    ("003", "expenses_suppliers", include_str!("../../migrations/003_expenses_suppliers.sql")),
    ("004", "users_permissions", include_str!("../../migrations/004_users_permissions.sql")),
    ("005", "reminders", include_str!("../../migrations/005_reminders.sql")),
    ("006", "indexes", include_str!("../../migrations/006_indexes.sql")),
    ("007", "pos_module", include_str!("../../migrations/007_pos_module.sql")),
    ("008", "normalize_units", include_str!("../../migrations/008_normalize_units.sql")),
];

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Create the migrations tracking table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS _schema_migrations (
            version TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            applied_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Bootstrap: if tables exist but no migrations are recorded, mark all as applied.
    // This handles existing databases that were created before versioned migrations.
    let (applied_count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM _schema_migrations"
    )
    .fetch_one(pool)
    .await?;

    if applied_count == 0 {
        // Check if this is an existing database (has the clients table)
        let (has_clients,): (bool,) = sqlx::query_as(
            "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'clients')"
        )
        .fetch_one(pool)
        .await?;

        if has_clients {
            // Existing database — mark all current migrations as applied
            for (version, name, _sql) in MIGRATIONS {
                sqlx::query(
                    "INSERT INTO _schema_migrations (version, name) VALUES ($1, $2) ON CONFLICT DO NOTHING"
                )
                .bind(version)
                .bind(name)
                .execute(pool)
                .await?;
            }
            return Ok(());
        }
    }

    // Get already-applied versions
    let applied: Vec<(String,)> = sqlx::query_as(
        "SELECT version FROM _schema_migrations ORDER BY version"
    )
    .fetch_all(pool)
    .await?;

    let applied_versions: std::collections::HashSet<&str> = applied
        .iter()
        .map(|(v,)| v.as_str())
        .collect();

    // Run pending migrations in order
    for (version, name, sql) in MIGRATIONS {
        if applied_versions.contains(version) {
            continue;
        }

        // Execute the migration SQL (may contain multiple statements)
        sqlx::raw_sql(sql)
            .execute(pool)
            .await?;

        // Record it as applied
        sqlx::query(
            "INSERT INTO _schema_migrations (version, name) VALUES ($1, $2)"
        )
        .bind(version)
        .bind(name)
        .execute(pool)
        .await?;
    }

    Ok(())
}
