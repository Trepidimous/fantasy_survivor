use yew::prelude::*;
use serde::{ Deserialize, Serialize };
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;

use crate::web_server::PLATFORM_URL;
use crate::logger;

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

	pub fn new(id_in : Option<i32>, name_in : String, id_showseason_in : Option<i32>) -> Self
	{
		LeagueState
		{
			id : id_in,
			name : name_in,
			id_showseason : id_showseason_in
		}
	}

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

#[derive(Clone)]
pub struct GameShowSystem
{
	pub gameshow_state: UseStateHandle<GameShowState>,
	pub gameshows: UseStateHandle<Vec<GameShow>>,
	pub get_gameshows: Callback<()>,
	pub create_gameshow: yew::Callback<yew::MouseEvent>,
	pub delete_gameshow: Callback<i32>,
	pub league_state : UseStateHandle<LeagueState>,
	pub leagues : UseStateHandle<Vec<League>>,
	pub collect_leagues: Callback<i32>,
	pub create_league: yew::Callback<i32>,
	pub delete_league: Callback<i32>,
	pub enter_user_into_league : Callback<(i32, i32)>,
	pub remove_user_from_league : yew::Callback<(i32, i32)>
}

pub fn collect_leagues(leagues_in: &UseStateHandle<Vec<League>>,
	message: &UseStateHandle<String>) -> Callback<i32>
{
	let leagues: UseStateHandle<Vec<League>> = leagues_in.clone();
	let message: UseStateHandle<String> = message.clone();
	Callback::from(move |id_showseason: i32|
	{
		let leagues: UseStateHandle<Vec<League>> = leagues.clone();
		let message: UseStateHandle<String> = message.clone();
		spawn_local(async move
		{
			logger::logger::log("Select League By ShowSeason-Id >>>".to_string() + id_showseason.to_string().as_str());

			let url:String = format!(concat!(PLATFORM_URL!(), "/leagues/from_season?id_showseason={}"), id_showseason);
			match Request::get(&url).send().await
			{
				Ok(resp) if resp.ok() =>
				{
					let fetched_leagues: Vec<League> = resp.json().await.unwrap_or_default();
					let num_leages = fetched_leagues.clone().len();
					leagues.set(fetched_leagues);
					message.set(format!("Successfully fetched all leagues with showseason[{}] num_leages[{}]", id_showseason, num_leages));
				}

				_ => message.set("Failed to fetch leagues".into()),
			}
		});
	})
}

pub fn create_league(league_state_in: &UseStateHandle<LeagueState>,
	message: &UseStateHandle<String>) -> Callback<i32>
{
	return
	{
		let league_state: UseStateHandle<LeagueState> = league_state_in.clone();
		let message: UseStateHandle<String> = message.clone();
		Callback::from(move |showseason_id: i32|
		{
			let league_state: UseStateHandle<LeagueState> = league_state.clone();
			let message: UseStateHandle<String> = message.clone();

			spawn_local(async move
			{
				logger::logger::log("create_league[FE] >>>".to_string() + league_state.name.as_str() + " ,,, " + showseason_id.to_string().as_str() );
				let league_data: serde_json::Value = serde_json::json!( { "name": league_state.name, "id_showseason": showseason_id } );
				let url:&str = concat!(PLATFORM_URL!(), "/leagues");
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::post(url)
					.header("Content-Type", "application/json")
					.body(league_data.to_string())
					.send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						message.set("League created successfully".into());
					}

					_ => message.set("Failed to create league".into()),
				}

				league_state.set(LeagueState::new(None, "".to_string(), None));
			});
		})
	};
}

pub fn delete_league(message: &UseStateHandle<String>) -> Callback<i32>
{
	return
	{
		let message: UseStateHandle<String> = message.clone();

		Callback::from(move |id: i32|
		{
			let message: UseStateHandle<String> = message.clone();
			spawn_local(async move
			{
				let url:String  = format!(concat!(PLATFORM_URL!(), "/leagues/{}"), id);
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::delete(&url).send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						message.set("League deleted successfully".into());
					}

					_ => message.set("Failed to delete league".into()),
				}
			});
		})
	};
}

pub fn enter_user_into_league(message: &UseStateHandle<String>) -> Callback<(i32, i32)>
{
	return
	{
		let message: UseStateHandle<String> = message.clone();
		Callback::from(move | (id_user, id_league) : (i32, i32) |
		{

			logger::logger::log("entering user >>>".to_string() + id_user.to_string().as_str() + " into league[" + id_league.to_string().as_str());
			let message: UseStateHandle<String> = message.clone();

			spawn_local(async move
			{
				let url:String = format!(concat!(PLATFORM_URL!(), "/leagues?user_id={}&league_id={}"), id_user, id_league);
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::post(&url)
					.header("Content-Type", "application/json")
					.send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						message.set(format!("Player [{}] entered successfully onto league [{}]", id_user.to_string(), id_league.to_string()).into());
					}

					_ => message.set(format!("Failed to enroll player[{}] onto league[{}]", id_user.to_string(), id_league.to_string()).into()),
				}
			});
		})
	};
}

fn remove_user_from_league(message: &UseStateHandle<String>) -> Callback<(i32, i32)>
{
	return
	{
		let message: UseStateHandle<String> = message.clone();
		Callback::from(move | (id_user, id_league) : (i32, i32) |
		{

			logger::logger::log("removing user >>>".to_string() + id_user.to_string().as_str() + " from league[" + id_league.to_string().as_str());
			let message: UseStateHandle<String> = message.clone();

			spawn_local(async move
			{
				let url:String = format!(concat!(PLATFORM_URL!(), "/leagues?user_id={}&league_id={}"), id_user, id_league);
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::delete(&url)
					.header("Content-Type", "application/json")
					.send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						message.set(format!("Player [{}] removed successfully from league [{}]", id_user.to_string(), id_league.to_string()).into());
					}

					_ => message.set(format!("Failed to remove player[{}] from league[{}]", id_user.to_string(), id_league.to_string()).into()),
				}
			});
		})
	};
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
	let collect_leagues: Callback<i32> = collect_leagues(&leagues, &message);
	let create_league: yew::Callback<i32> = create_league(&league_state, &message);
	let delete_league: Callback<i32> = delete_league(&message);
	let enter_user_into_league : yew::Callback<(i32, i32)> = enter_user_into_league(&message);
	let remove_user_from_league : yew::Callback<(i32, i32)> = remove_user_from_league(&message);

	return GameShowSystem { gameshow_state, gameshows, get_gameshows, create_gameshow, delete_gameshow,
		league_state, leagues, collect_leagues, create_league, delete_league, enter_user_into_league, remove_user_from_league };
}