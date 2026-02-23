use crate::gameshow_manager::League;
use crate::utilities::storage::StorageConnector;

use std::sync::Arc;

pub struct LeagueRepository
{
	connector: Arc<StorageConnector>,
}

impl LeagueRepository
{
	pub async fn new(storage_connection: Arc<StorageConnector>) -> Self
	{
		let league_repository: LeagueRepository = LeagueRepository
		{
			connector: Arc::clone(&storage_connection),
		};

		league_repository.initialize_storage_leagues().await;
		league_repository.initialize_storage_league_memberships().await;
		league_repository.initialize_storage_league_picks().await;

		return league_repository;
	}

	async fn initialize_storage_leagues(&self) -> ()
	{
		self.connector.storage
			.execute(
				"CREATE TABLE IF NOT EXISTS leagues (
					id SERIAL PRIMARY KEY,
					name TEXT NOT NULL,
					id_showseason INTEGER
				)",
				&[]
			).await
			.expect("Failed to create table");
	}

	async fn initialize_storage_league_memberships(&self) -> ()
	{
		self.connector.storage
			.execute(
				"CREATE TABLE IF NOT EXISTS league_members (
						id SERIAL PRIMARY KEY,
						league_id INTEGER NOT NULL REFERENCES leagues(id)
							ON DELETE CASCADE,
						user_id INTEGER NOT NULL REFERENCES users(id)
							ON DELETE CASCADE,
						UNIQUE(league_id, user_id)
				)",
				&[]
			).await
			.expect("Failed to create league_members table");
	}

	async fn initialize_storage_league_picks(&self) -> ()
	{
		self.connector.storage
			.execute(
				"CREATE TABLE IF NOT EXISTS league_picks (
						id SERIAL PRIMARY KEY,
						league_id INTEGER NOT NULL REFERENCES leagues(id)
							ON DELETE CASCADE,
						user_id INTEGER NOT NULL REFERENCES users(id)
							ON DELETE CASCADE,
						round_number INTEGER NOT NULL,
						contestant_id INTEGER NOT NULL REFERENCES contestants(contestant_id)
							ON DELETE CASCADE,
						rank_pick INTEGER NOT NULL,
						UNIQUE(league_id, user_id, round_number, contestant_id)
				)",
				&[]
			).await
			.expect("Failed to create league_picks table");
	}

	pub async fn collect_leagues(&self, id_show_season : i32) -> Result<Vec<League>, String>
	{
		let users: Vec<League> = self.connector.storage
			.query("SELECT id, name, id_showseason FROM leagues WHERE id_showseason = $1", &[&id_show_season]).await
			.map_err(|e: tokio_postgres::Error| e.to_string()) ?
			.iter()
			.map(|row: &tokio_postgres::Row| League { id: Some(row.get(0)), name: row.get(1), id_showseason: row.get(2) })
			.collect::<Vec<League>>();

		return Ok(users);
	}

	pub async fn create_league(&self, league: &League) -> Result<(), String>
	{
		println!("create_league[{}], [{}]", league.name, league.id_showseason.unwrap_or(-1));
		self.connector.storage
			.execute(
				"INSERT INTO leagues (name, id_showseason) VALUES ($1, $2)",
				&[&league.name, &league.id_showseason]
			).await
			.map_err(|e: tokio_postgres::Error| e.to_string())?;

		return Ok(());
	}

	pub async fn delete_league(&self, id: i32) -> Result<(), String>
	{
		self.connector.storage
			.execute("DELETE FROM leagues WHERE id = $1", &[&id]).await
			.map_err(|e|  e.to_string())?;

		return Ok(());
	}

	pub async fn add_user_to_league(&self, user_id: i32, league_id: i32) -> Result<(), String>
	{
		self.connector.storage
			.execute(
					"INSERT INTO league_members (league_id, user_id) VALUES ($1, $2)",
					&[&league_id, &user_id]
			).await
			.map_err(|e: tokio_postgres::Error| e.to_string())?;
		
		Ok(())
	}

	pub async fn remove_user_from_league(&self, user_id: i32, league_id: i32) -> Result<(), String>
	{
		self.connector.storage
			.execute(
					"DELETE FROM league_members WHERE user_id = $1 AND league_id = $2",
					&[&user_id, &league_id]
			).await
			.map_err(|e: tokio_postgres::Error| e.to_string())?;
		
		Ok(())
	}

	pub async fn set_league_pick(&self, league_id: i32, user_id: i32, round_number: i32, contestant_id: i32, rank_pick: i32) -> Result<(), String>
	{
		self.connector.storage
			.execute(
					"INSERT INTO league_picks (league_id, user_id, round_number, contestant_id, rank_pick) VALUES ($1, $2, $3, $4, $5)
					ON CONFLICT (league_id, user_id, contestant_id, round_number) DO UPDATE SET rank_pick = EXCLUDED.rank_pick",
					&[&league_id, &user_id, &round_number, &contestant_id, &rank_pick]
			).await
			.map_err(|e: tokio_postgres::Error| e.to_string())?;
		
		Ok(())
	}

}
