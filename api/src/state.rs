use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct AppState {
    db: Arc<DatabaseConnection>,
}

impl AppState {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub fn db_arc(&self) -> Arc<DatabaseConnection> {
        self.db.clone()
    }
}
