
use rocket::serde::{ Deserialize, Serialize };

use crate::memberships_accessor;

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

	pub async fn create() -> Self
	{
		// 2. Initialize the Resource Access Layer (Repo)
		let repository: memberships_accessor::UserRepository = memberships_accessor::UserRepository::connect_to().await;
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

	/*
	pub async fn add_user_and_refresh(&self, user: &GameShow) -> Result<Vec<GameShow>, String>
	{
		self.add_user(&user).await?;
		return self.collect_users().await;
	}

	async fn add_user(&self, user: &GameShow) -> Result<(), String>
	{
		return self.repo.add_user(user).await;
	}

	pub async fn delete_user_and_refresh(&self, id: i32) -> Result<Vec<GameShow>, String>
	{
		self.delete_user(id).await.map_err(|e| e.to_string())?;
		return self.collect_users().await;
	}

	async fn delete_user(&self, id: i32) -> Result<(), String>
	{
		return self.repo.delet_user(id).await;
	}
	*/
}