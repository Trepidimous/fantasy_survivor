use yew::prelude::*;
use serde::{ Deserialize, Serialize };
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;

use crate::web_server::PLATFORM_URL;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameShow
{
	pub id: i32,
	pub name: String,
}

#[derive(Clone, PartialEq)]
pub struct GameShowState
{
	pub name: String,
	pub id: Option<i32>,
}

impl GameShowState
{
	pub fn new(id_in : Option<i32>, name_in : String) -> Self
	{
		GameShowState
		{
			name : name_in,
			id : id_in
		}
	}

	pub fn from_default() -> Self
	{
		GameShowState
		{
			name: "".to_string(),
			id: None,
		}
	}
}

pub fn get_gameshows(gameshows: &UseStateHandle<Vec<GameShow>>,
	message: &UseStateHandle<String>) -> Callback<()>
{
	let gameshows: UseStateHandle<Vec<GameShow>> = gameshows.clone();
	let message: UseStateHandle<String> = message.clone();
	Callback::from(move |_|
	{
		let gameshows: UseStateHandle<Vec<GameShow>> = gameshows.clone();
		let message: UseStateHandle<String> = message.clone();
		spawn_local(async move
		{
			let url:&str = concat!(PLATFORM_URL!(), "/gameshows");
			match Request::get(&url).send().await
			{
				Ok(resp) if resp.ok() =>
				{
					let fetched_gameshows: Vec<GameShow> = resp.json().await.unwrap_or_default();
					gameshows.set(fetched_gameshows);
				}

				_ => message.set("Failed to fetch gameshows".into()),
			}
		});
	})
}

pub fn create_gameshow(gameshow_state: &UseStateHandle<GameShowState>,
	message: &UseStateHandle<String>,
	get_gameshows: Callback<()>) -> Callback<MouseEvent>
{
	return
	{
		let gameshow_state: UseStateHandle<GameShowState> = gameshow_state.clone();
		let message: UseStateHandle<String> = message.clone();
		let get_gameshows: Callback<()> = get_gameshows.clone();
		Callback::from(move |_|
		{
			let gameshow_state: UseStateHandle<GameShowState> = gameshow_state.clone();
			let message: UseStateHandle<String> = message.clone();
			let get_gameshows: Callback<()> = get_gameshows.clone();

			spawn_local(async move
			{
				let gameshow_data: serde_json::Value = serde_json::json!({ "name": gameshow_state.name });
				let url:&str = concat!(PLATFORM_URL!(), "/gameshows");
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::post(url)
					.header("Content-Type", "application/json")
					.body(gameshow_data.to_string())
					.send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						message.set("Game Show created successfully".into());
						get_gameshows.emit(());
					}

					_ => message.set("Failed to create game show".into()),
				}

				gameshow_state.set(GameShowState::new(None, "".to_string()));
			});
		})
	};
}

pub fn delete_gameshow(message: &UseStateHandle<String>,
	get_gameshows: Callback<()>) -> Callback<i32>
{
	return
	{
		let message: UseStateHandle<String> = message.clone();
		let get_gameshows: Callback<()> = get_gameshows.clone();

		Callback::from(move |id: i32|
		{
			let message: UseStateHandle<String> = message.clone();
			let get_gameshows: Callback<()> = get_gameshows.clone();

			spawn_local(async move
			{
				let url:String  = format!(concat!(PLATFORM_URL!(), "/gameshows/{}"), id);
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::delete(&url).send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						message.set("Game Show deleted successfully".into());
						get_gameshows.emit(());
					}

					_ => message.set("Failed to delete gameshow".into()),
				}
			});
		})
	};
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct League
{
	pub id: i32,
	pub name: String,
}

#[derive(Clone, PartialEq)]
pub struct LeagueState
{
	pub id: Option<i32>,
	pub name: String,
	pub id_showseason: Option<i32>
}

impl LeagueState
{
	pub fn from_default() -> Self
	{
		LeagueState
		{
			id : None,
			name : "".to_string(),
			id_showseason : None
		}
	}
}

pub struct GameShowSystem
{
	pub gameshow_state: UseStateHandle<GameShowState>,
	pub gameshows: UseStateHandle<Vec<GameShow>>,
	pub get_gameshows: Callback<()>,
	pub create_gameshow: yew::Callback<yew::MouseEvent>,
	pub delete_gameshow: Callback<i32>,
	pub league_state : UseStateHandle<LeagueState>,
	pub leagues : UseStateHandle<Vec<League>>,
	//pub collect_leagues: Callback<()>,
	//pub create_league: yew::Callback<yew::MouseEvent>,
	//pub delete_league: Callback<i32>,
}

#[hook]
pub fn use_compile_gameshow_system(message: UseStateHandle<String>) -> GameShowSystem
{
	let gameshow_state: UseStateHandle<GameShowState> = use_state(|| GameShowState::from_default());
	let gameshows: UseStateHandle<Vec<GameShow>> = use_state(Vec::new);

	let get_gameshows: Callback<()> = get_gameshows(&gameshows, &message);
	let create_gameshow: yew::Callback<yew::MouseEvent> = create_gameshow(&gameshow_state, &message, get_gameshows.clone());
	let delete_gameshow: Callback<i32> = delete_gameshow(&message, get_gameshows.clone());

	let league_state : UseStateHandle<LeagueState> = use_state(|| LeagueState::from_default());
	let leagues : UseStateHandle<Vec<League>> = use_state(Vec::new);

	return GameShowSystem { gameshow_state, gameshows, get_gameshows, create_gameshow, delete_gameshow, league_state, leagues };
}