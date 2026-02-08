#[macro_use]
extern crate rocket;

mod user_manager;
mod gameshows_accessor;
mod memberships_accessor;
mod league_accessor;
mod utilities;

mod gameshow_manager;

use rocket::serde::{ json::Json };
use rocket::{ State, response::status::Custom, http::Status };
use rocket_cors::{ CorsOptions, AllowedOrigins };

use crate::utilities::storage::StorageConnector;
use crate::gameshow_manager::{Contestant, GameShow, GameShowManager, League};
use crate::user_manager::User;
use crate::user_manager::UserManager;

use std::sync::Arc;

#[launch]
async fn rocket() -> _
{
	let storage_connection : StorageConnector = StorageConnector::establish_connection().await;
	let shared_storage: Arc<StorageConnector> = Arc::new(storage_connection);

	let memberships_repository : memberships_accessor::UserRepository = memberships_accessor::UserRepository::new(Arc::clone(&shared_storage)).await;
	let shared_memberships_repo : Arc<memberships_accessor::UserRepository> = Arc::new(memberships_repository);
	let gameshows_respository : gameshows_accessor::GameShowRepository = gameshows_accessor::GameShowRepository::new(Arc::clone(&shared_storage)).await;
	let shared_gameshows_repo : Arc<gameshows_accessor::GameShowRepository> = Arc::new(gameshows_respository);
	let league_repository : league_accessor::LeagueRepository = league_accessor::LeagueRepository::new(Arc::clone(&shared_storage)).await;
	let shared_leagues_repo : Arc<league_accessor::LeagueRepository> = Arc::new(league_repository);

	let user_manager: UserManager = UserManager::create(Arc::clone(&shared_memberships_repo)).await;
	let gameshow_manager : GameShowManager = GameShowManager::create(	Arc::clone(&shared_gameshows_repo),
																							Arc::clone(&shared_leagues_repo)
																						).await;

	let cors: rocket_cors::Cors = CorsOptions::default()
		.allowed_origins(AllowedOrigins::all())
		.to_cors()
		.expect("Error while building CORS");

	rocket::build()
		.manage(user_manager)
		.manage(gameshow_manager)
		.mount("/", routes![	add_user, collect_users, update_user, delete_user,
									collect_gameshows, add_gameshow, delete_gameshow,
									create_contestant, select_contestant_by_name, collect_contestants, delete_contestant,
									enroll_contestant,
									gameshow_preflight, gameshow_preflight_for_delete, create_contestant_preflight, delete_contestant_preflight])
		.attach(cors)
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

#[post("/api/gameshows", data = "<gameshow>")]
async fn add_gameshow(
	manager : &State<GameShowManager>,
	gameshow: Json<GameShow>
	) -> Result<Json<Vec<GameShow>>, Custom<String>>
{
	return manager.add_gameshow_and_refresh(&gameshow).await.map(Json).map_err(|e: String| Custom(Status::InternalServerError, e));
}

#[delete("/api/gameshows/<id>")]
async fn delete_gameshow(manager : &State<GameShowManager>, id: i32) -> Result<Json<Vec<GameShow>>, Custom<String>>
{
	return manager.delete_gameshow_and_refresh(id).await.map(Json).map_err(|e: String| Custom(Status::InternalServerError, e));
}

#[post("/api/contestants", data = "<contestant>")]
async fn create_contestant(
	manager : &State<GameShowManager>,
	contestant: Json<Contestant>
	) -> Result<Json<Contestant>, Custom<String>>
{
	let creation_result =manager.create_contestant(&contestant).await.map_err(|e: String| e);
	match creation_result
	{
		Ok(_created_contestant) =>
		{
			println!("Created contestant with name [{}]", contestant.name);
		}
		_=> println!("Failed to create contestant. Error: {}", creation_result.err().unwrap_or("Unknown error".to_string())),
	}
	let selection_result = manager.select_contestant_by_name(contestant.name.clone());
	let selection_result_json = selection_result.await.map(Json).map_err(|e: String| Custom(Status::InternalServerError, e));

	return selection_result_json;
}

#[get("/api/contestants/select?<name>")]
async fn select_contestant_by_name(
	manager : &State<GameShowManager>,
	name: String
	) -> Result<Json<Contestant>, Custom<String>>
{

	println!("SelConByNam>>>{}", name);

	return manager.select_contestant_by_name(name).await.map(Json).map_err(|e: String| Custom(Status::InternalServerError, e));
}

#[get("/api/contestants")]
async fn collect_contestants(
	manager : &State<GameShowManager>
	) -> Result<Json<Vec<Contestant>>, Custom<String>>
{
	return manager.collect_all_contestants().await.map(Json).map_err(|e: String| Custom(Status::InternalServerError, e));
}

#[delete("/api/contestants/<name>")]
async fn delete_contestant(manager : &State<GameShowManager>, name: &str) -> Result<(), String>
{
	return manager.delete_contestant(name).await.map_err(|e: String| e);
}

#[post("/api/contestants/enroll" , data = "<contestant>")]
async fn enroll_contestant(manager : &State<GameShowManager>, contestant: Json<Contestant>) -> Result<(), String>
{
	return manager.enter_contestant_onto_show(contestant.id.unwrap(), 
															contestant.id_showseason.unwrap(),
															contestant.nickname.clone().unwrap() ).await.map_err(|e: String| e);
}

#[get("/api/leagues/<id_showseason>")]
async fn collect_leagues(
	manager : &State<GameShowManager>,
	id_showseason : i32) -> Result<Json<Vec<League>>, Custom<String>>
{
	return manager.collect_leagues(id_showseason).await.map(Json).map_err(|e: String| Custom(Status::InternalServerError, e));
}

#[post("/api/leagues", data = "<league>")]
async fn create_league(
	manager : &State<GameShowManager>,
	league : Json<League>
	) -> Result<(), String>
{
	let creation_result = manager.create_league(&league).await.map_err(|e: String| e);
	return creation_result;
}

#[delete("/api/leagues/<league_id>/show/<id_seasonshow>")]
async fn delete_league(manager : &State<GameShowManager>, league_id: i32, id_seasonshow : i32) -> Result<Json<Vec<League>>, Custom<String>>
{
	let deletion_result = manager.delete_league(league_id).await.map_err(|e: String| e);
	return manager.collect_leagues(id_seasonshow).await.map(Json).map_err(|e: String| Custom(Status::InternalServerError, e));
}

///// These are just fake endpoints added in to stop server warnings //////

// Browsers automatically send out an options request before sending POST requests with Json payloads.
// This just lets browsers know that it is ok.
#[options("/api/gameshows")]
fn gameshow_preflight() -> Status
{
	Status::NoContent
}

#[options("/api/gameshows/<id>")]
fn gameshow_preflight_for_delete(id : i32) -> Status
{
	Status::NoContent
}

// I tried this adding this to stop a warning, but it didn't help.
#[options("/api/contestants")]
fn create_contestant_preflight(
	) -> Result<(), String>
{
	return Ok(());
}

#[options("/api/contestants/<name>")]
async fn delete_contestant_preflight(name: &str) -> Result<(), String>
{
	return Ok(());
}