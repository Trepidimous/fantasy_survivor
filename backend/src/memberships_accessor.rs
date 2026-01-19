
use crate::user_manager::User;
use crate::gameshow_manager::GameShow;
use crate::utility::StorageConnector;

use std::sync::Arc;

pub struct UserRepository
{
	connector: Arc<StorageConnector>,
}

impl UserRepository
{
	pub async fn new(storage_connection: Arc<StorageConnector>) -> Self
	{	
		let user_repository: UserRepository = UserRepository
		{
			connector: Arc::clone(&storage_connection),
		};

		return user_repository;
	}

	pub async fn initialize_user_storage(&self) -> ()
	{
		self.connector.storage
			.execute(
				"CREATE TABLE IF NOT EXISTS users (
					id SERIAL PRIMARY KEY,
					name TEXT NOT NULL,
					email TEXT NOT NULL,
					atype TEXT NOT NULL,
					num_league_tokens INTEGER DEFAULT 0
				)",
				&[]
			).await
			.expect("Failed to create table");

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

	pub async fn initialize_contestant_storage(&self) -> ()
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

	// Users //

	pub async fn collect_users(&self) -> Result<Vec<User>, String>
	{
		let users: Vec<User> = self.connector.storage
			.query("SELECT id, name, email, atype FROM users", &[]).await
			.map_err(|e: tokio_postgres::Error| e.to_string()) ?
			.iter()
			.map(|row: &tokio_postgres::Row| User { id: Some(row.get(0)), name: row.get(1), email: row.get(2), account_type : row.get(3) })
			.collect::<Vec<User>>();

		return Ok(users);
	}

	pub async fn add_user(&self, user: &User) -> Result<(), String>
	{
		self.connector.storage
			.execute(
				"INSERT INTO users (name, email, atype) VALUES ($1, $2, $3)",
				&[&user.name, &user.email, &user.account_type]
			).await
			.map_err(|e: tokio_postgres::Error| e.to_string())?;

		return Ok(());
	}

	pub async fn edit_user(&self, id: i32, user: &User) -> Result<(), String>
	{
		self.connector.storage.execute(
		"UPDATE users SET name = $1, email = $2 WHERE id = $3",
		&[&user.name, &user.email, &id]
		).await
		.map_err(|e: tokio_postgres::Error| e.to_string())?;
		
		return Ok(());
	}

	pub async fn delet_user(&self, id: i32) -> Result<(), String>
	{
		self.connector.storage
			.execute("DELETE FROM users WHERE id = $1", &[&id]).await
			.map_err(|e|  e.to_string())?;

		return Ok(());
	}

	// Game Shows //

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
