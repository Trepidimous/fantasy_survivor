
use crate::gameshow_manager::GameShow;
use crate::utilities::storage::StorageConnector;

use std::sync::Arc;

pub struct GameShowRepository
{
	connector: Arc<StorageConnector>,
}

impl GameShowRepository
{
	pub async fn new(storage_connection: Arc<StorageConnector>) -> Self
	{	
		let user_repository: GameShowRepository = GameShowRepository
		{
			connector: Arc::clone(&storage_connection),
		};

		return user_repository;
	}

	pub async fn initialize_storage(&self) -> ()
	{
		self.connector.storage
			.execute(
				"CREATE TABLE IF NOT EXISTS contestants (
					contestant_id SERIAL PRIMARY KEY,
					name TEXT NOT NULL
				)",
				&[]
			).await
			.expect("Failed to create table");

		self.connector.storage
			.execute(
				"CREATE TABLE IF NOT EXISTS game_show_contestants (
					contestant_id INTEGER,
					game_show_id INTEGER,
					PRIMARY KEY (contestant_id, game_show_id),
					FOREIGN KEY (contestant_id) REFERENCES contestants(contestant_id),
					FOREIGN KEY (game_show_id) REFERENCES game_shows(game_show_id)
				)",
				&[]
			).await
			.expect("Failed to create table");

	}

	pub async fn collect_game_shows(&self) -> Result<Vec<GameShow>, String>
	{
		let users: Vec<GameShow> = self.connector.storage
			.query("SELECT game_show_id, name FROM game_shows", &[]).await
			.map_err(|e: tokio_postgres::Error| e.to_string()) ?
			.iter()
			.map(|row: &tokio_postgres::Row| GameShow { id: Some(row.get(0)), name: row.get(1) })
			.collect::<Vec<GameShow>>();

		return Ok(users);
	}

	pub async fn add_gameshow(&self, game_show: &GameShow) -> Result<(), String>
	{
		self.connector.storage
			.execute(
				"INSERT INTO game_shows (name) VALUES ($1)",
				&[&game_show.name]
			).await
			.map_err(|e: tokio_postgres::Error| e.to_string())?;

		return Ok(());
	}

	pub async fn delete_game_show(&self, id: i32) -> Result<(), String>
	{
		self.connector.storage
			.execute("DELETE FROM game_shows WHERE game_show_id = $1", &[&id]).await
			.map_err(|e|  e.to_string())?;

		return Ok(());
	}

}
