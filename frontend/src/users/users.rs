use yew::prelude::*;
use serde::{ Deserialize, Serialize };
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;

use crate::web_server::PLATFORM_URL;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserState
{
	pub name: String,
	pub email: String,
	pub account_type: String,
	pub id: Option<i32>,
}

impl UserState
{
	pub fn from_default() -> Self
	{
		UserState
		{
			name: "".to_string(),
			email: "".to_string(),
			account_type: "".to_string(),
			id: None,
		}
	}

	pub fn new(id_in : Option<i32>, name_in : String, email_in : String, account_type_in : String) -> Self
	{
		UserState
		{
			name : name_in,
			email : email_in,
			account_type : account_type_in,
			id : id_in,
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User
{
	pub id: i32,
	pub name: String,
	pub email: String,
	pub account_type: String,
}

pub fn get_users(users: &UseStateHandle<Vec<User>>,
	message: &UseStateHandle<String>) -> Callback<()>
{
	let users: UseStateHandle<Vec<User>> = users.clone();
	let message: UseStateHandle<String> = message.clone();
	Callback::from(move |_|
	{
		let users: UseStateHandle<Vec<User>> = users.clone();
		let message: UseStateHandle<String> = message.clone();
		spawn_local(async move
		{
			let url = concat!(PLATFORM_URL!(), "/users");
			match Request::get(&url).send().await
			{
				Ok(resp) if resp.ok() =>
				{
					let fetched_users: Vec<User> = resp.json().await.unwrap_or_default();
					users.set(fetched_users);
				}

				_ => message.set("Failed to fetch users".into()),
			}
		});
	})
}

pub fn create_user(user_state: &UseStateHandle<UserState>, 
	message: &UseStateHandle<String>,
	get_users: Callback<()>) -> yew::Callback<yew::MouseEvent>
{
	return
	{
		let user_state: UseStateHandle<UserState> = user_state.clone();
		let message: UseStateHandle<String> = message.clone();
		let get_users: Callback<()> = get_users.clone();
		Callback::from(move |_|
		{
			let user_state: UseStateHandle<UserState> = user_state.clone();
			let message: UseStateHandle<String> = message.clone();
			let get_users: Callback<()> = get_users.clone();
			let account_type_in: &str = "Developer"; // GameMaster, Admin, Player

			spawn_local(async move
			{
				let user_data: serde_json::Value = serde_json::json!({ "name": user_state.name, "email": user_state.email, "account_type" : account_type_in });
				let url = concat!(PLATFORM_URL!(), "/users");
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::post(&url)
					.header("Content-Type", "application/json")
					.body(user_data.to_string())
					.send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						message.set("User created successfully".into());
						get_users.emit(());
					}

					_ => message.set("Failed to create user".into()),
				}

				user_state.set(UserState::from_default());
			});
		})
	};
}

pub fn update_user(user_state: &UseStateHandle<UserState>,
	message: &UseStateHandle<String>,
	get_users: Callback<()>) -> Callback<MouseEvent>
{
	return 
	{
		let user_state: UseStateHandle<UserState> = user_state.clone();
		let message: UseStateHandle<String> = message.clone();
		let get_users: Callback<()> = get_users.clone();

		Callback::from(move |_|
		{
			let editing_user_id: Option<i32> = user_state.id;
			let user_state: UseStateHandle<UserState> = user_state.clone();
			let message: UseStateHandle<String> = message.clone();
			let get_users: Callback<()> = get_users.clone();

			if let Some(id) = editing_user_id
			{
				spawn_local(async move
				{
					let url = format!(concat!(PLATFORM_URL!(), "/users/{}"), id);
					let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::put(&url)
						.header("Content-Type", "application/json")
						.body( serde_json::to_string(&(id, user_state.name.as_str(), user_state.email.as_str(), user_state.account_type.as_str() )).unwrap())
						.send().await;

					match response
					{
						Ok(resp) if resp.ok() =>
						{
							message.set("User updated successfully".into());
							get_users.emit(());
						}

						_ => message.set("Failed to update user".into()),
					}

					user_state.set(UserState::from_default());
				});
			}
		})
	};
}

pub fn delete_user(message: &UseStateHandle<String>,
	get_users: Callback<()>) -> Callback<i32>
{
	return
	{
		let message: UseStateHandle<String> = message.clone();
		let get_users: Callback<()> = get_users.clone();

		Callback::from(move |id: i32|
		{
			let message: UseStateHandle<String> = message.clone();
			let get_users: Callback<()> = get_users.clone();

			spawn_local(async move
			{
				let url: String = format!(concat!(PLATFORM_URL!(), "/users/{}"), id);
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::delete(&url).send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						message.set("User deleted successfully".into());
						get_users.emit(());
					}

					_ => message.set("Failed to delete user".into()),
				}
			});
		})
	};
}

pub fn edit_user(user_state : &UseStateHandle<UserState>, users : &UseStateHandle<Vec<User>>) -> Callback<i32>
{
	return
	{
		let user_state_handle: UseStateHandle<UserState> = user_state.clone();
		let users: UseStateHandle<Vec<User>> = users.clone();

		Callback::from(move |id: i32|
		{
			if let Some(user) = users.iter().find(|u: &&User| u.id == id)
			{
				let edited_user: UserState = UserState::new(
					Some(user.id),
					user.name.clone(),
					user.email.clone(),
					user.account_type.clone());

				user_state_handle.set(edited_user);
			}
		})
	};
}

#[derive(Clone)]
pub struct UserSystem
{
	pub user_state: UseStateHandle<UserState>,
	pub users: UseStateHandle<Vec<User>>,
	pub get_users: Callback<()>,
	pub create_user: yew::Callback<yew::MouseEvent>,
	pub update_user: Callback<MouseEvent>,
	pub delete_user: Callback<i32>,
	pub edit_user: Callback<i32>,
}

#[hook]
pub fn use_compile_user_system(message: UseStateHandle<String>) -> UserSystem
{
	let user_state: UseStateHandle<UserState> = use_state(|| UserState::from_default());
	let users: UseStateHandle<Vec<User>> = use_state(Vec::new);

	let get_users: Callback<()> = get_users(&users, &message);
	let create_user: yew::Callback<yew::MouseEvent> = create_user(&user_state, &message, get_users.clone());
	let update_user: Callback<MouseEvent> = update_user(&user_state, &message, get_users.clone());
	let delete_user: Callback<i32> = delete_user(&message, get_users.clone());
	let edit_user: Callback<i32> = edit_user(&user_state, &users);

	return UserSystem { user_state, users, get_users, create_user, update_user, delete_user, edit_user };
}