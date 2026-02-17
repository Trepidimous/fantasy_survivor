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
	pub id_showseason: Option<i32>,
	pub round_number : Option<i32>,
	pub was_medically_evacuated: Option<bool>
}

impl ContestantState
{
	pub fn new(id_in : Option<i32>, name_in : String, id_showseason_in: Option<i32>) -> Self
	{
		ContestantState
		{
			name : name_in,
			id : id_in,
			id_showseason : id_showseason_in,
			round_number: Some(-1),
			was_medically_evacuated: Some(false)
		}
	}

	pub fn to_string(&self) -> String
	{
		return format!("ContestantState {{ id: {:?}, name: {}, id_showseason: {:?} }}, round_#: [{:?}], Medical[: {:?}]",
			self.id,
			self.name,
			self.id_showseason,
			self.round_number.unwrap_or_default(),
			self.was_medically_evacuated.unwrap_or_default());
	}

	pub fn convert_to_json(&self) -> serde_json::Value
	{
		let json = serde_json::json!(
		{
			"name": self.name,
			"id": self.id,
			"id_showseason": self.id_showseason,
			"round_number": self.round_number.unwrap_or_default(),
			"was_medically_evacuated": self.was_medically_evacuated.unwrap_or_default()
		});

		return json;
	}
}

fn create_contestant(contestant_state: &UseStateHandle<ContestantState>,
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
				let contestant_data: serde_json::Value = serde_json::json!({ 
					"name": contestant_state.name,
					"round_number": contestant_state.round_number,
					"was_medically_evacuated": contestant_state.was_medically_evacuated
				});

				let url:&str = concat!(PLATFORM_URL!(), "/contestants");
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::post(url)
					.header("Content-Type", "application/json")
					.body(contestant_data.to_string())
					.send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						if let Ok(json) = resp.json::<serde_json::Value>().await
						{
							let id = json.get("id").and_then(|v| v.as_i64()).map(|v| v as i32);
							let name = json.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
							let id_showseason = json.get("id_showseason").and_then(|v| v.as_i64()).map(|v| v as i32);

							contestant_state.set(ContestantState::new(id, name.clone(), id_showseason));
							message.set(format!("Contestant[{}] created successfully. With have ID[{}]", name, id.unwrap_or(-1)));
						}
					}

					_ => message.set("Failed to create contestant".into()),
				}

			});
		})
	};
}

fn select_contestant_by_name(
	contestant_state: &UseStateHandle<ContestantState>,
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

				logger::logger::log("Select Contestant By Name >>>".to_string() + contestant_state.name.to_string().as_str());

				let url: String = format!(concat!(PLATFORM_URL!(), "/contestants/select?name={}"), contestant_state.name);
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::get(&url)
					.send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						if let Ok(json) = resp.json::<serde_json::Value>().await
						{
							let id = json.get("id").and_then(|v| v.as_i64()).map(|v| v as i32);
							let name = json.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
							let id_showseason = json.get("id_showseason").and_then(|v| v.as_i64()).map(|v| v as i32);

							contestant_state.set(ContestantState::new(id, name.clone(), id_showseason));
							message.set(format!("Contestant[{}] selected successfully. They have ID[{}]", name, id.unwrap_or(-1)));
						}
					}

					_ => message.set("Failed to select contestant".into()),
				}
			});
		})
	};
}

fn delete_contestant(contestant_state: &UseStateHandle<ContestantState>,
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
						message.set(format!("Contestant[{}] deleted successfully", contestant_state.name));
					}

					_ => message.set("Failed to delete contestant".into()),
				}
			});
		})
	};
}

fn fetch_contestants_on_show(message: &UseStateHandle<String>) -> Callback<i32>
{
	return
	{
		let message: UseStateHandle<String> = message.clone();
		Callback::from(move |game_show_id: i32|
		{
			let message: UseStateHandle<String> = message.clone();

			spawn_local(async move
			{
				let url: String = format!(concat!(PLATFORM_URL!(), "/contestants/on_show?game_show_id={}"), game_show_id);
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::get(&url).send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						if let Ok(json) = resp.json::<serde_json::Value>().await
						{
							message.set(format!("Fetched contestants on show successfully. ShowID[{}], Response: {}", game_show_id, json.to_string()));
						}
					}

					_ => message.set("Failed to fetch contestants on show".into()),
				}
			});
		})
	};
}

fn enroll_contestant_onto_show(
	message: &UseStateHandle<String>) -> Callback<ContestantState>
{
	return
	{
		let message: UseStateHandle<String> = message.clone();
		Callback::from(move |incoming_state: ContestantState|
		{

			logger::logger::log("Enrolling >>>".to_string() + incoming_state.to_string().as_str());
			let message: UseStateHandle<String> = message.clone();

			spawn_local(async move
			{
				let contestant_data: serde_json::Value = incoming_state.convert_to_json();
				let url:&str = concat!(PLATFORM_URL!(), "/contestants/enroll");
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::post(url)
					.header("Content-Type", "application/json")
					.body(contestant_data.to_string())
					.send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						message.set(format!("Contestant enrolled successfully. [{}]", incoming_state.to_string()).into());
					}

					_ => message.set("Failed to enroll contestant onto show".into()),
				}
			});
		})
	};
}

fn eliminiate_contestant_from_show(message: &UseStateHandle<String>) -> Callback<ContestantState>
{
	return
	{
		let message: UseStateHandle<String> = message.clone();
		Callback::from(move |incoming_state: ContestantState|
		{
			logger::logger::log("Eliminating >>>".to_string() + incoming_state.to_string().as_str());
			let message: UseStateHandle<String> = message.clone();

			spawn_local(async move
			{
				let contestant_data: serde_json::Value = incoming_state.convert_to_json();
				let url:&str = concat!(PLATFORM_URL!(), "/contestants/elim");
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::post(url)
					.header("Content-Type", "application/json")
					.body(contestant_data.to_string())
					.send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						message.set(format!("The Tribe Has Spoken 💨. [{}]", incoming_state.to_string()).into());
					}

					_ => message.set("Failed to eliminate contestant from show".into()),
				}
			});
		})
	};
}

fn medevac_contestant(message: &UseStateHandle<String>) -> Callback<ContestantState>
{
	return
	{
		let message: UseStateHandle<String> = message.clone();
		Callback::from(move |incoming_state: ContestantState|
		{

			logger::logger::log("MedEvacing >>>".to_string() + incoming_state.to_string().as_str());
			let message: UseStateHandle<String> = message.clone();

			spawn_local(async move
			{
				let contestant_data: serde_json::Value = incoming_state.convert_to_json();
				let url:&str = concat!(PLATFORM_URL!(), "/contestants/medevac");
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::post(url)
					.header("Content-Type", "application/json")
					.body(contestant_data.to_string())
					.send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						message.set(format!("Contestant medevaced successfully. [{}]", incoming_state.to_string()).into());
					}

					_ => message.set("Failed to medevac contestant from show".into()),
				}
			});
		})
	};
}


#[derive(Clone)]
pub struct ContestantSystem
{
	pub contestant_state : UseStateHandle<ContestantState>,
	pub create_contestant: yew::Callback<yew::MouseEvent>,
	pub select_contestant: yew::Callback<yew::MouseEvent>,
	pub delete_contestant: Callback<String>,
	pub fetch_contestants_on_show : Callback<i32>,
	pub enroll_contestant_onto_show: Callback<ContestantState>,
	pub eliminate_contestant_from_show: Callback<ContestantState>,
	pub medevac_contestant_from_show: Callback<ContestantState>
}

#[hook]
pub fn use_compile_contestant_system(message: UseStateHandle<String>) -> ContestantSystem
{
	let contestant_state : UseStateHandle<ContestantState> = use_state(|| ContestantState { name: "".to_string(), id: None, id_showseason: None, round_number : Some(-1), was_medically_evacuated: Some(false) });

	let create_contestant : yew::Callback<yew::MouseEvent> = create_contestant(&contestant_state, &message);
	let select_contestant : yew::Callback<yew::MouseEvent> = select_contestant_by_name(&contestant_state, &message);
	let delete_contestant : Callback<String> = delete_contestant(&contestant_state, &message);
	let fetch_contestants_on_show : Callback<i32> = fetch_contestants_on_show(&message);
	let enroll_contestant_onto_show : Callback<ContestantState> = enroll_contestant_onto_show(&message);
	let eliminate_contestant_from_show : Callback<ContestantState> = eliminiate_contestant_from_show(&message);
	let medevac_contestant_from_show : Callback<ContestantState> = medevac_contestant(&message);

	return ContestantSystem { contestant_state, create_contestant, select_contestant, delete_contestant,
		fetch_contestants_on_show,
		enroll_contestant_onto_show, eliminate_contestant_from_show, medevac_contestant_from_show };
}