
use rocket::serde::{ Deserialize, Serialize };

use crate::memberships_accessor;
use crate::utility::StorageConnector;

use std::sync::Arc;

pub struct GameShowManager
{
	pub repo: memberships_accessor::UserRepository,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameShow
{
	pub id: Option<i32>,
	pub name: String,
}

impl GameShowManager
{

	pub async fn create(storage_connection : Arc<StorageConnector>) -> Self
	{
		// 2. Initialize the Resource Access Layer (Repo)
		let repository: memberships_accessor::UserRepository = memberships_accessor::UserRepository::connect_to(storage_connection).await;
		repository.initialize_storage().await;
		
		let game_repository: GameShowManager = GameShowManager
		{
			repo : repository
		};

		return game_repository;
	}

	pub async fn collect_gameshows(&self) -> Result<Vec<GameShow>, String>
	{
		return self.repo.collect_game_shows().await;
	}

	pub async fn add_gameshow_and_refresh(&self, gameshow: &GameShow) -> Result<Vec<GameShow>, String>
	{
		self.repo.add_gameshow(&gameshow).await?;
		return self.repo.collect_game_shows().await;
	}

	pub async fn delete_gameshow_and_refresh(&self, id: i32) -> Result<Vec<GameShow>, String>
	{
		self.delete_gameshow(id).await.map_err(|e| e.to_string())?;
		return self.collect_gameshows().await;
	}

	async fn delete_gameshow(&self, id: i32) -> Result<(), String>
	{
		return self.repo.delete_game_show(id).await;
	}
	
}