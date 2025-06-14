// File: crates/core/src/database/connection.rs
// Path: finalverse/crates/core/src/database/connection.rs
// Description: Database connection pool management using r2d2 and diesel.
//              Provides thread-safe database access across all services.

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool, PoolError, PooledConnection};
use std::env;
use std::time::Duration;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
}

impl DatabaseConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "32".to_string())
                .parse()
                .unwrap_or(32),
            min_connections: env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .unwrap_or(2),
            connection_timeout: Duration::from_secs(
                env::var("DB_CONNECTION_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
            ),
            idle_timeout: Duration::from_secs(
                env::var("DB_IDLE_TIMEOUT")
                    .unwrap_or_else(|_| "600".to_string())
                    .parse()
                    .unwrap_or(600),
            ),
            max_lifetime: Duration::from_secs(
                env::var("DB_MAX_LIFETIME")
                    .unwrap_or_else(|_| "1800".to_string())
                    .parse()
                    .unwrap_or(1800),
            ),
        })
    }
}

/// Create a new database connection pool
pub fn create_connection_pool(config: &DatabaseConfig) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);

    Pool::builder()
        .max_size(config.max_connections)
        .min_idle(Some(config.min_connections))
        .connection_timeout(config.connection_timeout)
        .idle_timeout(Some(config.idle_timeout))
        .max_lifetime(Some(config.max_lifetime))
        .build(manager)
}

/// Database connection pool manager
pub struct DatabaseManager {
    pool: DbPool,
}

impl DatabaseManager {
    /// Create a new database manager
    pub fn new(config: &DatabaseConfig) -> Result<Self, PoolError> {
        let pool = create_connection_pool(config)?;
        Ok(Self { pool })
    }

    /// Get a connection from the pool
    pub fn get_connection(&self) -> Result<DbConnection, PoolError> {
        self.pool.get()
    }

    /// Get pool statistics
    pub fn pool_stats(&self) -> PoolStats {
        let state = self.pool.state();
        PoolStats {
            connections: state.connections,
            idle_connections: state.idle_connections,
            max_connections: self.pool.max_size(),
        }
    }

    /// Run database migrations
    #[cfg(feature = "migrations")]
    pub fn run_migrations(&self) -> Result<(), diesel_migrations::RunMigrationsError> {
        use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

        let mut conn = self.get_connection()
            .map_err(|e| diesel_migrations::RunMigrationsError::QueryError(
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string()),
                )
            ))?;

        conn.run_pending_migrations(MIGRATIONS)?;
        Ok(())
    }
}

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub connections: u32,
    pub idle_connections: u32,
    pub max_connections: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_config_from_env() {
        // Set test environment variables
        env::set_var("DATABASE_URL", "postgres://test:test@localhost/test");
        env::set_var("DB_MAX_CONNECTIONS", "50");

        let config = DatabaseConfig::from_env().unwrap();
        assert_eq!(config.database_url, "postgres://test:test@localhost/test");
        assert_eq!(config.max_connections, 50);
    }
}