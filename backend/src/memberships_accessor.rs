
use crate::user_manager::User;
use crate::utilities::storage::StorageConnector;

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

}
