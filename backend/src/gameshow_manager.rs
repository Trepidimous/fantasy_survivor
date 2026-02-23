use rocket::serde::{ Deserialize, Serialize };

use crate::{gameshows_accessor, league_accessor};

use std::sync::Arc;

pub struct GameShowManager
{
	pub repo: Arc<gameshows_accessor::GameShowRepository>,
	pub league_repository: Arc<league_accessor::LeagueRepository>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameShow
{
	pub id: Option<i32>,
	pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Contestant
{
	pub id: Option<i32>,
	pub name: String,
	pub id_showseason: Option<i32>,
	pub nickname: Option<String>,
	pub round_number: i32,
	pub was_medically_evacuated: bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct League
{
	pub id: Option<i32>,
	pub name: String,
	pub id_showseason: Option<i32>
}

impl GameShowManager
{
	pub async fn create(		repository : Arc<gameshows_accessor::GameShowRepository>, 
								league_repository_in : Arc<league_accessor::LeagueRepository>) -> Self
	{	
		let game_repository: GameShowManager = GameShowManager
		{
			repo : repository,
			league_repository : league_repository_in
		};

		return game_repository;
	}

	pub async fn collect_gameshows(&self) -> Result<Vec<GameShow>, String>
	{
		return self.repo.collect_game_shows().await;
	}

	pub async fn add_gameshow_and_refresh(&self, gameshow: &GameShow) -> Result<Vec<GameShow>, String>
	{
		self.repo.add_gameshow(&gameshow).await?;
		return self.repo.collect_game_shows().await;
	}

	pub async fn delete_gameshow_and_refresh(&self, id: i32) -> Result<Vec<GameShow>, String>
	{
		self.delete_gameshow(id).await.map_err(|e| e.to_string())?;
		return self.collect_gameshows().await;
	}

	pub async fn delete_gameshow(&self, id: i32) -> Result<(), String>
	{
		return self.repo.delete_game_show(id).await;
	}

	pub async fn create_contestant(&self, contestant: &Contestant) -> Result<(), String>
	{
		return self.repo.create_contestant(contestant).await;
	}

	pub async fn select_contestant_by_name(&self, name: String) -> Result<Contestant, String>
	{
		return self.repo.select_contestant_by_name(name).await;
	}

	pub async fn delete_contestant(&self, name: &str) -> Result<(), String>
	{
		return self.repo.delete_contestant(name).await;
	}

	pub async fn collect_all_contestants(&self) -> Result<Vec<Contestant>, String>
	{
		return self.repo.collect_all_contestants().await;
	}

	pub async fn enter_contestant_onto_show(&self, contestant_id: i32, game_show_id: i32, nickname: String) -> Result<(), String>
	{
		return self.repo.enter_contestant_onto_show(contestant_id, game_show_id, nickname).await;
	}

	pub async fn eliminiate_contestant_from_show(&self, contestant_id: i32, game_show_id: i32, round_number : i32) -> Result<(), String>
	{
		return self.repo.eliminate_contestant_from_show(contestant_id, game_show_id, round_number).await;
	}

	pub async fn medically_evacuate_contestant_from_show(&self, contestant_id: i32, game_show_id: i32, round_number : i32) -> Result<(), String>
	{
		let elimination_result = self.repo.eliminate_contestant_from_show(contestant_id, game_show_id, round_number).await;
		if elimination_result.is_err()
		{
			return Err(elimination_result.err().unwrap_or("Unable to Eliminate Candidate".to_string()));
		}

		return self.repo.medically_evacuate_contestant_from_show(contestant_id, game_show_id).await;
	}

	pub async fn create_league(&self, league: &League) -> Result<(), String>
	{
		println!("G.S.M. create_league[{}], [{}]", league.name, league.id_showseason.unwrap_or(-1));
		return self.league_repository.create_league(league).await;
	}

	pub async fn collect_leagues(&self, id_showseason : i32) -> Result<Vec<League>, String>
	{
		return self.league_repository.collect_leagues(id_showseason).await;
	}

	pub async fn delete_league(&self, id: i32) -> Result<(), String>
	{
		return self.league_repository.delete_league(id).await;
	}

	pub async fn add_user_to_league(&self, user_id: i32, league_id: i32) -> Result<(), String>
	{
		return self.league_repository.add_user_to_league(user_id, league_id).await;
	}

	pub async fn remove_user_from_league(&self, user_id: i32, league_id: i32) -> Result<(), String>
	{
		return self.league_repository.remove_user_from_league(user_id, league_id).await;
	}

	pub async fn fetch_contestants_on_show(&self, game_show_id: i32) -> Result<Vec<Contestant>, String>
	{
		return self.repo.fetch_contestants_on_show(game_show_id).await;
	}

	pub async fn set_league_pick(&self, league_id: i32, user_id: i32, round_number: i32, contestant_id: i32, rank_pick: i32) -> Result<(), String>
	{
		return self.league_repository.set_league_pick(league_id, user_id, round_number, contestant_id, rank_pick).await;
	}

}