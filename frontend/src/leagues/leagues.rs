use yew::prelude::*;
use serde::{ Deserialize, Serialize };
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;

use crate::web_server::PLATFORM_URL;
use crate::logger;

const player_id : i32 = 1;
const league_id : i32 = 1;

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
	pub picks_state: UseStateHandle<Vec<RoundPickState>>,
	pub submit_picks : yew::Callback<yew::MouseEvent>
}

pub fn submit_picks(message: &UseStateHandle<String>, picks: &UseStateHandle<Vec<RoundPickState>>) -> yew::Callback<yew::MouseEvent>
{
	return
	{
		let picks_state: UseStateHandle<Vec<RoundPickState>> = picks.clone();
		let message: UseStateHandle<String> = message.clone();

		Callback::from(move |_|
		{
			let picks_state: UseStateHandle<Vec<RoundPickState>> = picks_state.clone();
			let message: UseStateHandle<String> = message.clone();

			spawn_local(async move
			{

				logger::logger::log(format!("submit_picks >>> Submitting picks for league [{}]", league_id.to_string()));

				for (round_pick_state) in (picks_state).iter()
				{
					for (contestant_pick) in round_pick_state.picks.iter()
					{
						logger::logger::log(format!("Submitting pick for contestant [{}] with rank [{}]", contestant_pick.contestant_id, contestant_pick.rank_pick));

						let url:String = format!(concat!(PLATFORM_URL!(), "/leagues/set_pick?user_id={}&league_id={}&round_number={}&contestant_id={}&rank_pick={}"), player_id, league_id, round_pick_state.round_number, contestant_pick.contestant_id, contestant_pick.rank_pick);
						let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::post(&url)
							.header("Content-Type", "application/json")
							.send().await;

						match response
						{
							Ok(resp) if resp.ok() =>
							{
								message.set(format!("Pick [{}] entered successfully onto league [{}]", contestant_pick.rank_pick, league_id.to_string()).into());
							}

							_ => message.set(format!("Failed to enter pick [{}] onto league [{}]", contestant_pick.rank_pick, league_id.to_string()).into()),
						}
					}
				}



			});
		})
	};
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
	let picks_state: UseStateHandle<Vec<RoundPickState>> = use_state(Vec::new);

	let submit_picks: yew::Callback<yew::MouseEvent> = submit_picks(&message, &picks_state);

	return LeagueSystem { picks_state, submit_picks };
}