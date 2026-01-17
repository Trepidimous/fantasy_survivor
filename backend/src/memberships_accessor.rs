
use tokio_postgres::{ Client, NoTls };

use crate::user_manager::User;

pub struct UserRepository
{
	client: Client,
}

impl UserRepository
{
	pub async fn connect_to() -> Self
	{
		let (new_client, connection) = tokio_postgres
			::connect("host=localhost user=postgres password=postgres dbname=postgres", NoTls).await
			.expect("Failed to connect to Postgres");

		tokio::spawn(async move
		{
			if let Err(e) = connection.await
			{
				eprintln!("Failed to connect to Postgres: {}", e);
			}
		});
		
		let user_repository: UserRepository = UserRepository
		{
			client: new_client,
		};

		return user_repository;
	}

	pub async fn initialize_storage(&self) -> ()
	{
		//Create the table if it doesn't exist
		// [todo] add league tokens like this: "league_token INTEGER"
		self.client
			.execute(
				"CREATE TABLE IF NOT EXISTS users (
					id SERIAL PRIMARY KEY,
					name TEXT NOT NULL,
					email TEXT NOT NULL,
					atype TEXT NOT NULL
				)",
				&[]
			).await
			.expect("Failed to create table");
	}

	pub async fn collect_users(&self) -> Result<Vec<User>, String>
	{
		return self.get_users_from_rocket_database().await;
	}

	async fn get_users_from_rocket_database(&self) -> Result<Vec<User>, String>
	{
		let users: Vec<User> = self.client
			.query("SELECT id, name, email, atype FROM users", &[]).await
			.map_err(|e: tokio_postgres::Error| e.to_string()) ?
			.iter()
			.map(|row: &tokio_postgres::Row| User { id: Some(row.get(0)), name: row.get(1), email: row.get(2), account_type : row.get(3) })
			.collect::<Vec<User>>();

		return Ok(users);
	}

	pub async fn add_user(&self, user: &User) -> Result<(), String>
	{
		self.client
			.execute(
				"INSERT INTO users (name, email, atype) VALUES ($1, $2, $3)",
				&[&user.name, &user.email, &user.account_type]
			).await
			.map_err(|e: tokio_postgres::Error| e.to_string())?;

		return Ok(());
	}

	pub async fn edit_user(&self, id: i32, user: &User) -> Result<(), String>
	{
		self.client.execute(
		"UPDATE users SET name = $1, email = $2 WHERE id = $3",
		&[&user.name, &user.email, &id]
		).await
		.map_err(|e: tokio_postgres::Error| e.to_string())?;
		
		return Ok(());
	}

	pub async fn delet_user(&self, id: i32) -> Result<(), String>
	{
		self.client
			.execute("DELETE FROM users WHERE id = $1", &[&id]).await
			.map_err(|e|  e.to_string())?;

		return Ok(());
	}

}
