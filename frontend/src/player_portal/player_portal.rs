use yew::prelude::*;
use crate::users::users::*;
use crate::gameshows::gameshows::*;
use crate::contestants::contestants::*;
use crate::leagues::leagues::*;
use crate::logger;

// Testing - To be replaced with login screen //
const player_id : i32 = 1;
const league_id : i32 = 1;
const game_show_id : i32 = 1;

pub fn build_player_portal_page(
	message: &UseStateHandle<String>,
	contestant_system : &ContestantSystem,
	dragged_index: &UseStateHandle<Option<usize>>,
	ranked_list: &UseStateHandle<Vec<ContestantState>>,
	league_system: &LeagueSystem
) -> Html
{

	let ranked_list_for_submit: UseStateHandle<Vec<ContestantState>> = (*ranked_list).clone();

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

				<ul class="space-y-2">
				{
					for (*ranked_list).iter().enumerate().map(|(index, contestant)|
					{
						let ondragstart =
						{
							let dragged_index = dragged_index.clone();
							Callback::from(move |_| dragged_index.set(Some(index)))
						};

						let ondragover = Callback::from(|e: DragEvent| e.prevent_default());

						let ondrop =
						{
							let dragged_index = dragged_index.clone();
							let ranked_list = ranked_list.clone();
							Callback::from(move |e: DragEvent|
							{
								e.prevent_default();
								if let Some(from_idx) = *dragged_index
								{
									let mut new_vec = (*ranked_list).clone();
									// Manual reorder logic
									let item = new_vec.remove(from_idx);
									new_vec.insert(index, item);
									ranked_list.set(new_vec);
								}
								dragged_index.set(None);
							})
						};

						html!
						{
							<li
								{ondragstart} {ondragover} {ondrop} draggable="true"
								class={classes!(
										"flex", "items-center", "p-4", "mb-2", "rounded-lg", "cursor-grab", 
										"border", "transition-all", "group",
										"w-[250px]",
										{ "bg-[#1e1e1e] border-gray-800" },
										"hover:border-[#4a90e2]"
								)}
							>

								<div class="flex-grow text-center">
										<span class="text-white font-semibold text-lg">{ &contestant.name }</span>
								</div>

								<div class="text-gray-600 group-hover:text-[#4a90e2] font-mono">
										{"⠿"}
								</div>
							</li>
						}
					})
				}
				</ul>

				<div class="mt-6">
					<button
						onclick={league_system.set_picks.reform(move |_| (league_id, player_id, 1, ranked_list_for_submit[0].id.unwrap_or_default(), 1))}
						class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded">
						{ "Submit Picks" }
					</button>
				</div>

			</div>
		</body>
	}
}
