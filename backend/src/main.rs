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
	pub async fn collect_users(&self) -> Result<Vec<User>, String> // Result<Json<Vec<User>>, Custom<String>>
	{
		self.get_users_from_rocket_database(&self.client).await//.map(Json)
	}

	async fn get_users_from_rocket_database(&self, client: &Client) -> Result<Vec<User>, String>
	{
		let users: Vec<User> = client
			.query("SELECT id, name, email, atype FROM users", &[]).await
			.map_err(|e: tokio_postgres::Error| e.to_string()) ?
			.iter()
			.map(|row: &tokio_postgres::Row| User { id: Some(row.get(0)), name: row.get(1), email: row.get(2), account_type : row.get(3) })
			.collect::<Vec<User>>();

		Ok(users)
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
		self.repo.collect_users().await
	}
}

#[get("/api/users")]
async fn collect_users(
	manager : &State<UserManager>
	) -> Result<Json<Vec<User>>, Custom<String>>
{
	manager.collect_users().await.map(Json).map_err(|e: String| Custom(Status::InternalServerError, e))
}

#[post("/api/users", data = "<user>")]
async fn add_user(
	conn: &State<Client>,
	user: Json<User>
	) -> Result<Json<Vec<User>>, Custom<String>>
{
	execute_query(
		conn,
		"INSERT INTO users (name, email, atype) VALUES ($1, $2, $3)",
		&[&user.name, &user.email, &user.account_type]
	).await?;
	get_users(conn).await
}

#[put("/api/users/<id>", data = "<user>")]
async fn update_user(
	conn: &State<Client>,
	id: i32,
	user: Json<User>
	) -> Result<Json<Vec<User>>, Custom<String>>
{
	execute_query(
		conn,
		"UPDATE users SET name = $1, email = $2 WHERE id = $3",
		&[&user.name, &user.email, &id]
	).await?;
	get_users(conn).await
}

#[delete("/api/users/<id>")]
async fn delete_user(conn: &State<Client>, id: i32) -> Result<Status, Custom<String>>
{
	execute_query(conn, "DELETE FROM users WHERE id = $1", &[&id]).await?;
	Ok(Status::NoContent)
}

async fn execute_query(
	client: &Client,
	query: &str,
	params: &[&(dyn tokio_postgres::types::ToSql + Sync)]
	) -> Result<u64, Custom<String>>
{
	client
		.execute(query, params).await
		.map_err(|e: tokio_postgres::Error | Custom(Status::InternalServerError, e.to_string()))
}

#[launch]
async fn rocket() -> _
{
	let (client, connection) = tokio_postgres
		::connect("host=localhost user=postgres password=postgres dbname=postgres", NoTls).await
		.expect("Failed to connect to Postgres");

	tokio::spawn(async move
	{
		if let Err(e) = connection.await
		{
			eprintln!("Failed to connect to Postgres: {}", e);
		}
	});

	//Create the table if it doesn't exist
	// [todo] add league tokens like this: "league_token INTEGER"
	client
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

    // 2. Initialize the Resource Access Layer (Repo)
    let repo: UserRepository = UserRepository { client };

    // 3. Initialize the Business Layer (Manager)
    let user_manager: UserManager = UserManager { repo };

	let cors: rocket_cors::Cors = CorsOptions::default()
		.allowed_origins(AllowedOrigins::all())
		.to_cors()
		.expect("Error while building CORS");

	rocket
		::build()
		.manage(client)
		.manage(user_manager)
		.mount("/", routes![add_user, collect_users, update_user, delete_user])
		.attach(cors)

}