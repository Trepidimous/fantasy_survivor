use yew::prelude::*;
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;

use crate::web_server::PLATFORM_URL;

pub struct ContestantState
{
	pub name: String,
	pub id: Option<i32>,
}

impl ContestantState
{
	pub fn new(id_in : Option<i32>, name_in : String) -> Self
	{
		ContestantState
		{
			name : name_in,
			id : id_in
		}
	}
}

pub fn create_contestant(contestant_state: &UseStateHandle<ContestantState>,
	message: &UseStateHandle<String>) -> yew::Callback<yew::MouseEvent>
{
	return
	{
		let contestant_state: UseStateHandle<ContestantState> = contestant_state.clone();
		let message: UseStateHandle<String> = message.clone();
		Callback::from(move |_|
		{
			let contestant_state: UseStateHandle<ContestantState> = contestant_state.clone();
			let message: UseStateHandle<String> = message.clone();

			spawn_local(async move
			{
				let contestant_data: serde_json::Value = serde_json::json!({ "name": contestant_state.name });
				let url:&str = concat!(PLATFORM_URL!(), "/contestants");
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::post(url)
					.header("Content-Type", "application/json")
					.body(contestant_data.to_string())
					.send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						message.set("Contestant created successfully".into());
					}

					_ => message.set("Failed to create contestant".into()),
				}

				contestant_state.set(ContestantState::new(None, "".to_string()));
			});
		})
	};
}

pub fn delete_contestant(contestant_state: &UseStateHandle<ContestantState>,
	message: &UseStateHandle<String>) -> Callback<String>
{
	return
	{
		let message: UseStateHandle<String> = message.clone();
		let contestant_state: UseStateHandle<ContestantState> = contestant_state.clone();
		Callback::from(move |_|
		{
			let message: UseStateHandle<String> = message.clone();
			let contestant_state: UseStateHandle<ContestantState> = contestant_state.clone();

			spawn_local(async move
			{
				let url: String = format!(concat!(PLATFORM_URL!(), "/contestants/{}"), contestant_state.name);
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::delete(&url).send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						message.set("Contestant deleted successfully".into());
					}

					_ => message.set("Failed to delete contestant".into()),
				}
			});
		})
	};
}
