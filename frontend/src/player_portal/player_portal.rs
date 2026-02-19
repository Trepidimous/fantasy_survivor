use yew::prelude::*;

use crate::users::users::UserState;
use crate::users::users::*;
use crate::gameshows::gameshows::*;
use crate::contestants::contestants::*;
use crate::logger;


// Testing - To be replaced with login screen //
const player_id : i32 = 1;
const league_id : i32 = 1;
const game_show_id : i32 = 1;

pub fn build_player_portal_page(
	message: &UseStateHandle<String>,
	user_system : &UserSystem,
	gameshow_system : &GameShowSystem,
	contestant_system : &ContestantSystem
) -> Html
{
	html!
	{
		<body class="bg-[#121212]  min-h-screen">
			<div class="container mx-auto p-4">
				<h1 class="text-4xl font-bold text-[#FF8C00] mb-4">{ "Survivor Fantasy League" }</h1>

				<button
					onclick={contestant_system.fetch_contestants_on_show.reform(|_| (game_show_id))}
					class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded mb-4">
					{ "Fetch Contestants" }
				</button>

				if !message.is_empty()
				{
					<p class="text-green-500 mt-2">{ &**message }</p>
				}

				<ul class="list-disc pl-5">
				{
					for (*contestant_system.contestants_on_show).iter().map(|contestant|
						{
							let contestant_id = contestant.id.unwrap_or(-1);
							html!
							{
								<li class="mb-2">
								<span class="font-semibold text-[#4a90e2]">{ format!("ID: {}, Name: {} ", contestant_id, contestant.name) }</span>

								<button
									onclick={gameshow_system.enter_user_into_league.clone().reform(move |_| (contestant_id, league_id) )}
									class="ml-4 bg-green-500 hover:bg-green-700 text-white font-bold py-1 px-2 rounded">
									{ "Contestant" }
								</button>

								</li>
							}
						}
					)
				}
				</ul>


			</div>
		</body>
	}
}
