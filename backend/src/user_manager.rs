
use rocket::{ response::status::Custom, http::Status };

use crate::memberships_resource;
use crate::memberships_resource::User;

pub struct UserManager
{
	pub repo: memberships_resource::UserRepository,
}

impl UserManager
{
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
		self.delete_user(id).await.map_err(|e| e.0.to_string())?;
		return self.collect_users().await;
	}

	async fn delete_user(&self, id: i32) -> Result<Status, Custom<String>>
	{
		return self.repo.delet_user(id).await;
	}

}