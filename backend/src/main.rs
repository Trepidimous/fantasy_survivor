#[macro_use]
extern crate rocket;

mod memberships_resource;

use rocket::serde::{ Deserialize, Serialize, json::Json };
use rocket::{ State, response::status::Custom, http::Status };
use tokio_postgres::{ Client, NoTls };
use rocket_cors::{ CorsOptions, AllowedOrigins };

use crate::memberships_resource::User;


pub struct UserManager
{
	repo: memberships_resource::UserRepository,
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
	memberships_resource::helper_function();

	// 2. Initialize the Resource Access Layer (Repo)
	let repo: memberships_resource::UserRepository = memberships_resource::UserRepository::connect_to().await;

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