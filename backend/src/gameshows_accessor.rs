
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
		self.initialize_gameshows().await;

		self.initialize_contestants().await;

		self.initialize_gameshow_contestants().await;
	}

	async fn initialize_gameshows(&self)
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
	}

	async fn initialize_contestants(&self)
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
	}

	async fn initialize_gameshow_contestants(&self)
	{
		self.connector.storage
			.execute(
				"CREATE TABLE IF NOT EXISTS game_show_contestants (
				contestant_id INTEGER,
				game_show_id INTEGER,
				nickname TEXT,
				PRIMARY KEY (contestant_id, game_show_id),
				FOREIGN KEY (contestant_id) REFERENCES contestants(contestant_id)
					ON DELETE CASCADE,
				FOREIGN KEY (game_show_id) REFERENCES game_shows(game_show_id)
					ON DELETE CASCADE,
				was_medically_evacuated BOOLEAN DEFAULT FALSE,
				eliminated_on_round INTEGER DEFAULT -1
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
			.map_err(|e| {
			print!("delete_game_show error[{}]", e.to_string());
			e.to_string()
	})?;
		

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
					nickname: None,
					round_number: -1,
					was_medically_evacuated: false
				};

				return Ok(contestant);
			}

			None =>
			{
				return Err("Contestant not found".to_string());
			}
		}
	}

	pub async fn delete_contestant(&self, name: &str) -> Result<(), String>
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
			.map(|row: &tokio_postgres::Row| Contestant { id: Some(row.get(0)), name: row.get(1), id_showseason: None, nickname: None, round_number: -1, was_medically_evacuated: false })
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

	pub async fn eliminate_contestant_from_show(&self, contestant_id: i32, game_show_id: i32, round_number: i32) -> Result<(), String>
	{
		self.connector.storage
			.execute(
				"UPDATE game_show_contestants SET eliminated_on_round = $1 WHERE contestant_id = $2 AND game_show_id = $3",
				&[&round_number, &contestant_id, &game_show_id]
			).await
			.map_err(|e: tokio_postgres::Error| e.to_string())?;

		return Ok(());
	}

	pub async fn medically_evacuate_contestant_from_show(&self, contestant_id: i32, game_show_id: i32) -> Result<(), String>
	{
		self.connector.storage
			.execute(
				"UPDATE game_show_contestants SET was_medically_evacuated = TRUE WHERE contestant_id = $1 AND game_show_id = $2",
				&[&contestant_id, &game_show_id]
			).await
			.map_err(|e: tokio_postgres::Error| e.to_string())?;

		return Ok(());
	}

	pub async fn fetch_contestants_on_show(&self, game_show_id: i32) -> Result<Vec<Contestant>, String>
	{
		let contestants: Vec<Contestant> = self.connector.storage
			.query(
				"SELECT c.contestant_id, c.name, gsc.nickname, gsc.eliminated_on_round, gsc.was_medically_evacuated
				FROM contestants c
				JOIN game_show_contestants gsc ON c.contestant_id = gsc.contestant_id
				WHERE gsc.game_show_id = $1",
				&[&game_show_id]
			).await
			.map_err(|e: tokio_postgres::Error| e.to_string()) ?
			.iter()
			.map(|row: &tokio_postgres::Row| Contestant {
				id: Some(row.get(0)),
				name: row.get(1),
				nickname: row.get(2),
				round_number: row.get(3),
				was_medically_evacuated: row.get(4),
				id_showseason: Some(game_show_id)
			})
			.collect::<Vec<Contestant>>();

		return Ok(contestants);
	}

}
