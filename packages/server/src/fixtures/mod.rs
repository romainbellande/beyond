use mongodb::{Database};
mod planet;

pub async fn execute_fixtures(db: Database) {
  planet::fixture(db).await;
}
