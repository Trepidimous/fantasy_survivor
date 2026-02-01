use yew::prelude::*;
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;

use crate::web_server::PLATFORM_URL;

use crate::logger;

#[derive(Clone, PartialEq)]
pub struct ContestantState
{
	pub name: String,
	pub id: Option<i32>,
	pub id_showseason: Option<i32>
}

impl ContestantState
{
	pub fn new(id_in : Option<i32>, name_in : String, id_showseason_in: Option<i32>) -> Self
	{
		ContestantState
		{
			name : name_in,
			id : id_in,
			id_showseason : id_showseason_in
		}
	}
}

impl ContestantState
{
	pub fn to_string(&self) -> String
	{
		return format!("ContestantState {{ id: {:?}, name: {}, id_showseason: {:?} }}",
			self.id,
			self.name,
			self.id_showseason);
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

pub fn enroll_contestant_onto_show(
	message: &UseStateHandle<String>) -> Callback<ContestantState>
{

	// needs valid contestant id //

	return
	{
		let message: UseStateHandle<String> = message.clone();
		Callback::from(move |incoming_state: ContestantState|
		{

			logger::logger::log("Enrolling >>>".to_string() + incoming_state.to_string().as_str());
			let message: UseStateHandle<String> = message.clone();

			spawn_local(async move
			{
				let contestant_data: serde_json::Value = serde_json::json!({
					"name": incoming_state.name,
					"id": incoming_state.id,
					"id_showseason": incoming_state.id_showseason,
					"nickname": "" 
				});
				let url:&str = concat!(PLATFORM_URL!(), "/contestants/enroll");
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::post(url)
					.header("Content-Type", "application/json")
					.body(contestant_data.to_string())
					.send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						message.set("Contestant enrolled onto show successfully".into());
					}

					_ => message.set("Failed to enroll contestant onto show".into()),
				}
			});
		})
	};
}

pub struct ContestantSystem
{
	pub contestant_state : UseStateHandle<ContestantState>,
	pub create_contestant: yew::Callback<yew::MouseEvent>,
	pub delete_contestant: Callback<String>,
	pub enroll_contestant_onto_show: Callback<ContestantState>,
}

#[hook]
pub fn use_compile_contestant_system(message: UseStateHandle<String>) -> ContestantSystem
{
	let contestant_state : UseStateHandle<ContestantState> = use_state(|| ContestantState { name: "".to_string(), id: None, id_showseason: None });

	let create_contestant : yew::Callback<yew::MouseEvent> = create_contestant(&contestant_state, &message);
	let delete_contestant : Callback<String> = delete_contestant(&contestant_state, &message);
	let enroll_contestant_onto_show : Callback<ContestantState> = enroll_contestant_onto_show(&message);

	return ContestantSystem { contestant_state, create_contestant, delete_contestant, enroll_contestant_onto_show };
}