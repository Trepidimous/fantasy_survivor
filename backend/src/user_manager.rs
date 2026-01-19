
use rocket::serde::{ Deserialize, Serialize };

use crate::memberships_accessor;

use crate::utility::StorageConnector;

use std::sync::Arc;

pub struct UserManager
{
	pub repo: memberships_accessor::UserRepository,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User
{
	pub id: Option<i32>,
	pub name: String,
	pub email: String,
	pub account_type: String,
}

impl UserManager
{
	pub async fn create(storage_connection : Arc<StorageConnector>) -> Self
	{
		// 2. Initialize the Resource Access Layer (Repo)
		let repository: memberships_accessor::UserRepository = memberships_accessor::UserRepository::connect_to(storage_connection).await;
		repository.initialize_storage().await;

		let user_repository: UserManager = UserManager
		{
			repo : repository
		};

		return user_repository;
	}

	pub async fn collect_users(&self) -> Result<Vec<User>, String>
	{
		return self.repo.collect_users().await;
	}

	pub async fn add_user_and_refresh(&self, user: &User) -> Result<Vec<User>, String>
	{
		self.add_user(&user).await?;
		return self.collect_users().await;
	}

	async fn add_user(&self, user: &User) -> Result<(), String>
	{
		return self.repo.add_user(user).await;
	}

	pub async fn edit_user_and_refresh(&self, id: i32, user: &User) -> Result<Vec<User>, String>
	{
		self.edit_user(id, user).await?;
		return self.collect_users().await;
	}

	async fn edit_user(&self, id: i32, user: &User) -> Result<(), String>
	{
		return self.repo.edit_user(id, user).await;
	}

	pub async fn delete_user_and_refresh(&self, id: i32) -> Result<Vec<User>, String>
	{
		self.delete_user(id).await.map_err(|e| e.to_string())?;
		return self.collect_users().await;
	}

	async fn delete_user(&self, id: i32) -> Result<(), String>
	{
		return self.repo.delet_user(id).await;
	}

}