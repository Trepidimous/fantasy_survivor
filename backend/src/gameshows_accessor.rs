
use crate::utilities::storage::StorageConnector;

use crate::gameshow_manager::GameShow;
use crate::gameshow_manager::Contestant;

use std::sync::Arc;

pub struct GameShowRepository
{
	connector: Arc<StorageConnector>,
}

impl GameShowRepository
{
	pub async fn new(storage_connection: Arc<StorageConnector>) -> Self
	{	
		let repository: GameShowRepository = GameShowRepository
		{
			connector: Arc::clone(&storage_connection),
		};

		repository.initialize_storage().await;

		return repository;
	}

	async fn initialize_storage(&self) -> ()
	{
		self.connector.storage
			.execute(
				"CREATE TABLE IF NOT EXISTS game_shows (
					game_show_id SERIAL PRIMARY KEY,
					name TEXT DEFAULT 'Jeffs Jamboree'
				)",
				&[]
			).await
			.expect("Failed to create table");

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
					nickname TEXT,
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

	pub async fn create_contestant(&self, contestant: &Contestant) -> Result<(), String>
	{
		self.connector.storage
			.execute(
				"INSERT INTO contestants (name) VALUES ($1)",
				&[&contestant.name]
			).await
			.map_err(|e: tokio_postgres::Error| e.to_string())?;

		return Ok(());
	}

	pub async fn select_contestant_by_name(&self, name: String) -> Result<Contestant, String>
	{
		let row_option: Option<tokio_postgres::Row> = self.connector.storage
			.query_opt(
				"SELECT contestant_id, name FROM contestants WHERE name = $1",
				&[&name]
			).await
			.map_err(|e: tokio_postgres::Error| e.to_string())?;

		match row_option
		{
			Some(row) =>
			{
				let contestant: Contestant = Contestant
				{
					id: Some(row.get(0)),
					name: row.get(1),
					id_showseason: None,
					nickname: None
				};

				return Ok(contestant);
			}

			None =>
			{
				return Err("Contestant not found".to_string());
			}
		}
	}

	pub async fn delete_contestant(&self, name: String) -> Result<(), String>
	{
		self.connector.storage
			.execute(
				"DELETE FROM contestants WHERE name = $1",
				&[&name]
			).await
			.map_err(|e: tokio_postgres::Error| e.to_string())?;

		return Ok(());
	}

	pub async fn collect_all_contestants(&self) -> Result<Vec<Contestant>, String>
	{
		let users: Vec<Contestant> = self.connector.storage
			.query("SELECT contestant_id, name FROM contestants", &[]).await
			.map_err(|e: tokio_postgres::Error| e.to_string()) ?
			.iter()
			.map(|row: &tokio_postgres::Row| Contestant { id: Some(row.get(0)), name: row.get(1), id_showseason: None, nickname: None })
			.collect::<Vec<Contestant>>();

		return Ok(users);
	}

	pub async fn enter_contestant_onto_show(&self, contestant_id: i32, game_show_id: i32, nickname: String) -> Result<(), String>
	{
		self.connector.storage
			.execute(
				"INSERT INTO game_show_contestants (contestant_id, game_show_id, nickname) VALUES ($1, $2, $3)",
				&[&contestant_id, &game_show_id, &nickname]
			).await
			.map_err(|e: tokio_postgres::Error| e.to_string())?;

		return Ok(());
	}

}
