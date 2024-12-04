use csci211_project_two::{config, db, game};
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::load()?;

    let pool = MySqlPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await?;

    let db = db::Database::new(pool);
    let mut game_manager = game::GameManager::new(db);

    game_manager.run().await?;

    Ok(())
}
