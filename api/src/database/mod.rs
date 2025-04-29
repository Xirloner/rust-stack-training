use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;
use tracing::error;

pub async fn connect_to_database() -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new("protocol://username:password@host/database".to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    let db = Database::connect(opt).await.map_err(|e| {
        error!("Failed to connect to the database: {}", e);
        e
    })?;

    Ok(db)
}
