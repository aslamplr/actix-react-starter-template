use entity::sea_orm::DatabaseConnection;

use crate::services::cake::{CakeService, PgCakeService};

#[derive(Debug, Clone)]
pub struct AppContext {
    pub db: DatabaseConnection,
}

impl AppContext {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn cake_service(&self) -> impl CakeService + '_ {
        PgCakeService::spawn(&self.db)
    }
}
