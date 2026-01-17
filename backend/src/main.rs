#[macro_use]
extern crate rocket;

use rocket::serde::{ Deserialize, Serialize, json::Json };
use rocket::{ State, response::status::Custom, http::Status };
use tokio_postgres::{ Client, NoTls };
use rocket_cors::{ CorsOptions, AllowedOrigins };

#[derive(Serialize, Deserialize, Clone)]
pub struct User
{
	id: Option<i32>,
	name: String,
	email: String,
	account_type: String,
}

pub struct UserRepository
{
	client: Client,
}

impl UserRepository
{
	async fn connect_to() -> Self
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
		
		let user_repository = UserRepository
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

	async fn add_user(&self, user: &User) -> Result<(), String>
	{
		self.client
			.execute(
				"INSERT INTO users (name, email, atype) VALUES ($1, $2, $3)",
				&[&user.name, &user.email, &user.account_type]
			).await
			.map_err(|e: tokio_postgres::Error| e.to_string())?;

		return Ok(());
	}

	async fn edit_user(&self, id: i32, user: &User) -> Result<(), String>
	{
		self.client.execute(
		"UPDATE users SET name = $1, email = $2 WHERE id = $3",
		&[&user.name, &user.email, &id]
		).await
		.map_err(|e: tokio_postgres::Error| e.to_string())?;
		
		return Ok(());
	}

	async fn delet_user(&self, id: i32) -> Result<Status, Custom<String>>
	{
		self.client
			.execute("DELETE FROM users WHERE id = $1", &[&id]).await
			.map_err(|e: tokio_postgres::Error| Custom(Status::InternalServerError, e.to_string()))?;

		return Ok(Status::NoContent);
	}

}

pub struct UserManager
{
	repo: UserRepository,
}

impl UserManager
{
	pub async fn collect_users(&self) -> Result<Vec<User>, String>
	{
		return self.repo.collect_users().await;
	}

	pub async fn add_user(&self, user: &User) -> Result<(), String>
	{
		return self.repo.add_user(user).await;
	}

	pub async fn edit_user(&self, id: i32, user: &User) -> Result<(), String>
	{
		return self.repo.edit_user(id, user).await;
	}

	pub async fn delete_user(&self, id: i32) -> Result<Status, Custom<String>>
	{
		return self.repo.delet_user(id).await;
	}
}

#[get("/api/users")]
async fn collect_users(
	manager : &State<UserManager>
	) -> Result<Json<Vec<User>>, Custom<String>>
{
	return manager.collect_users().await.map(Json).map_err(|e: String| Custom(Status::InternalServerError, e));
}

#[post("/api/users", data = "<user>")]
async fn add_user(
	manager : &State<UserManager>,
	user: Json<User>
	) -> Result<Json<Vec<User>>, Custom<String>>
{
	manager.add_user(&user).await.map_err(|e: String| Custom(Status::InternalServerError, e))?;
	return manager.collect_users().await.map(Json).map_err(|e: String| Custom(Status::InternalServerError, e))
}

#[put("/api/users/<id>", data = "<user>")]
async fn update_user(
	manager : &State<UserManager>,
	id: i32,
	user: Json<User>
	) -> Result<Json<Vec<User>>, Custom<String>>
{
	manager.edit_user(id, &user).await.map_err(|e: String| Custom(Status::InternalServerError, e))?;
	return manager.collect_users().await.map(Json).map_err(|e: String| Custom(Status::InternalServerError, e));
}

#[delete("/api/users/<id>")]
async fn delete_user(manager : &State<UserManager>, id: i32) -> Result<Json<Vec<User>>, Custom<String>>
{
	manager.delete_user(id).await.map_err(|e: Custom<String>| e)?;
	return manager.collect_users().await.map(Json).map_err(|e: String| Custom(Status::InternalServerError, e));
}

#[launch]
async fn rocket() -> _
{
	// 2. Initialize the Resource Access Layer (Repo)
	let repo: UserRepository = UserRepository::connect_to().await;

	repo.initialize_storage().await;

	// 3. Initialize the Business Layer (Manager)
	let user_manager: UserManager = UserManager { repo };

	let cors: rocket_cors::Cors = CorsOptions::default()
		.allowed_origins(AllowedOrigins::all())
		.to_cors()
		.expect("Error while building CORS");

	rocket::build()
		.manage(user_manager)
		.mount("/", routes![add_user, collect_users, update_user, delete_user])
		.attach(cors)
}