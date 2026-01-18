#[macro_use]
extern crate rocket;

mod user_manager;
mod memberships_accessor;

mod gameshow_manager;

use rocket::serde::{ json::Json };
use rocket::{ State, response::status::Custom, http::Status };
use rocket_cors::{ CorsOptions, AllowedOrigins };

use crate::gameshow_manager::{GameShow, GameShowManager};
use crate::user_manager::User;
use crate::user_manager::UserManager;


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
	return manager.add_user_and_refresh(&user).await.map(Json).map_err(|e: String| Custom(Status::InternalServerError, e));
}

#[put("/api/users/<id>", data = "<user>")]
async fn update_user(
	manager : &State<UserManager>,
	id: i32,
	user: Json<User>
	) -> Result<Json<Vec<User>>, Custom<String>>
{
	return manager.edit_user_and_refresh(id, &user).await.map(Json).map_err(|e: String| Custom(Status::InternalServerError, e))
}

#[delete("/api/users/<id>")]
async fn delete_user(manager : &State<UserManager>, id: i32) -> Result<Json<Vec<User>>, Custom<String>>
{
	return manager.delete_user_and_refresh(id).await.map(Json).map_err(|e: String| Custom(Status::InternalServerError, e));
}

#[get("/api/gameshows")]
async fn collect_gameshows(
	manager : &State<GameShowManager>
	) -> Result<Json<Vec<GameShow>>, Custom<String>>
{
	return manager.collect_gameshows().await.map(Json).map_err(|e: String| Custom(Status::InternalServerError, e));
}

#[launch]
async fn rocket() -> _
{
	let user_manager: UserManager = UserManager::create().await;
	let gameshow_manager : GameShowManager = GameShowManager::create().await;

	let cors: rocket_cors::Cors = CorsOptions::default()
		.allowed_origins(AllowedOrigins::all())
		.to_cors()
		.expect("Error while building CORS");

	rocket::build()
		.manage(user_manager)
		.manage(gameshow_manager)
		.mount("/", routes![add_user, collect_users, update_user, delete_user, collect_gameshows])
		.attach(cors)
}