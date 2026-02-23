use yew::prelude::*;
use serde::{ Deserialize, Serialize };
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;

use crate::web_server::PLATFORM_URL;
use crate::logger;


#[derive(Clone, PartialEq)]
pub struct ContestantPickState
{
	pub contestant_id: i32,
	pub rank_pick: i32
}

impl ContestantPickState
{
	pub fn new(contestant_id_in : i32, rank_pick_in : i32) -> Self
	{
		ContestantPickState
		{
			contestant_id : contestant_id_in,
			rank_pick : rank_pick_in
		}
	}

	pub fn from_default() -> Self
	{
		ContestantPickState
		{
			contestant_id : -1,
			rank_pick : -1
		}
	}
}

pub struct RoundPickState
{
	pub round_number: i32,
	pub picks: Vec<ContestantPickState>
}

#[derive(Clone)]
pub struct LeagueSystem
{
	pub picks: UseStateHandle<Vec<RoundPickState>>,
	pub set_pick : Callback<(i32, i32, i32, i32, i32)>
}

pub fn set_pick(message: &UseStateHandle<String>) -> Callback<(i32, i32, i32, i32, i32)>
{
	return
	{
		let message: UseStateHandle<String> = message.clone();
		Callback::from(move | (id_league, id_user, round_number, id_contestant, rank_pick) : (i32, i32, i32, i32, i32) |
		{
			logger::logger::log("set_picks >>>".to_string() + " into league[" + id_league.to_string().as_str() + id_user.to_string().as_str());
			let message: UseStateHandle<String> = message.clone();

			spawn_local(async move
			{
				let url:String = format!(concat!(PLATFORM_URL!(), "/leagues/set_pick?user_id={}&league_id={}&round_number={}&contestant_id={}&rank_pick={}"), id_user, id_league, round_number, id_contestant, rank_pick);
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

#[hook]
pub fn use_create_league_system(message: UseStateHandle<String>) -> LeagueSystem
{
	let picks: UseStateHandle<Vec<RoundPickState>> = use_state(Vec::new);

	let set_pick : yew::Callback<(i32, i32, i32, i32, i32)> = set_pick(&message);

	return LeagueSystem { picks, set_pick };
}