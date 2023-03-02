use std::error::Error;

use diesel::pg::Pg;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};

pub fn run_migrations(
    connection: &mut impl MigrationHarness<Pg>,
    migrations: EmbeddedMigrations,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    connection.run_pending_migrations(migrations)?;

    Ok(())
}
