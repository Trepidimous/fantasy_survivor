use yew::prelude::*;

mod web_server;
mod users;
mod gameshows;
mod contestants;
mod logger;

use crate::users::users::UserState;
use crate::users::users::*;
use crate::gameshows::gameshows::*;
use crate::contestants::contestants::*;

fn main()
{
	yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html
{
	let message: UseStateHandle<String> = use_state(|| "".to_string());

	let user_system = users::users::use_compile_user_system(message.clone());
	let gameshow_system = gameshows::gameshows::use_compile_gameshow_system(message.clone());
	let contestant_system = contestants::contestants::use_compile_contestant_system(message.clone());

	build_website(&message, &user_system,
		&gameshow_system,
		&contestant_system)
}

fn build_website(
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
				<h1 class="text-4xl font-bold text-[#FF8C00] mb-4">{ "Game Master Portal" }</h1>
			{
				build_showseason_mangement(gameshow_system, contestant_system)
			}

			{
				build_user_management( message, user_system)
			}

			{
				build_league_management(gameshow_system)
			}

			</div>
		</body>
	}
}

fn build_league_management(gameshow_system : &GameShowSystem) -> Html
{
	let gameshow_state_clone: UseStateHandle<GameShowState> = gameshow_system.gameshow_state.clone();
	let league_state_clone: UseStateHandle<LeagueState> = gameshow_system.league_state.clone();

	let on_select_league: Callback<Event> = 
	{
		Callback::from(move |e: Event|
		{
			let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
			let value_string = input.value();
			
			let value: i32 = value_string.parse().unwrap_or(-1);

			let mut gameshow_state_update: GameShowState = (*gameshow_state_clone).clone();
			gameshow_state_update.id = Some(value);
			gameshow_state_clone.set(gameshow_state_update);

			let mut league_state_update: LeagueState = (*league_state_clone).clone();
			league_state_update.id = Some(value);
			league_state_clone.set(league_state_update);
			
			let output = format!("Selected League ID [{}]", value);
			logger::logger::log(output);
		})
	};

	let showseason_id = gameshow_system.gameshow_state.id.unwrap_or(-1);
	html!
	{
		<>

			<input placeholder="[new league name]"
				value={gameshow_system.league_state.name.clone()}
				oninput={Callback::from(
				{
					let league_state_clone = gameshow_system.league_state.clone();
					move |e: InputEvent|
					{
						let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();

						let edited_league: LeagueState = LeagueState::new(
							league_state_clone.id,
							input.value(),
							league_state_clone.id_showseason
						);

						league_state_clone.set(edited_league);
					}
				})}
				class="border rounded px-4 py-2 mr-2"
			/>

			<button
				onclick=
				{
					gameshow_system.create_league.clone().reform(move |_| showseason_id)
				}
				class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
				{
					"Create League"
				}
			</button>

			<button
				onclick={gameshow_system.delete_league.clone().reform(
				{
					let league_state_clone = gameshow_system.league_state.clone();
					move |_| league_state_clone.id.unwrap_or(-1)
				})}

				class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded">
				{
					"Delete League"
				}
			</button>

			<div class="mb-4">
				<button
					onclick={gameshow_system.collect_leagues.clone().reform(move |_| showseason_id )}
					class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded mb-4">
					{ "Fetch Leagues" }
				</button>
			</div>

			<div class="mb-4">
				<select onchange={on_select_league}>
					<option value="" disabled=true selected=true>{"Select a league"}</option>
					{
						gameshow_system.leagues.iter().map(|league|
						{
							html!
							{
								<option key={league.id} value={league.id.to_string()}>
								{
									&league.name
								}
								</option>
							}
						}).collect::<Html>()
					}
				</select>
			</div>

		</>
	}
}

fn build_showseason_mangement(
	gameshow_system : &GameShowSystem,
	contestant_system : &ContestantSystem
) -> Html
{

	let gameshow_state_clone: UseStateHandle<GameShowState> = gameshow_system.gameshow_state.clone();

	let on_select_showseason: Callback<Event> = 
	{
		Callback::from(move |e: Event| 
		{
			let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
			let value_string = input.value();
			
			let value: i32 = value_string.parse().unwrap_or(-1);

			let mut gameshow_state_update = (*gameshow_state_clone).clone();
			gameshow_state_update.id = Some(value);
			gameshow_state_clone.set(gameshow_state_update);
			
			let output = format!("Selected ShowSeason ID: {}", value);
			logger::logger::log(output);
		})
	};



	html!
	{
		<>
			<button
				onclick={gameshow_system.get_gameshows.reform(|_| ())}
				class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded mb-4">
				{ "Fetch Game Show Seasons" }
			</button>

			<input placeholder="New Game Show Season Name"
				value={gameshow_system.gameshow_state.name.clone()}
				oninput={Callback::from(
				{
					let gameshow_state_clone = gameshow_system.gameshow_state.clone();
					move |e: InputEvent|
					{
						let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();

						let edited_gameshow = GameShowState::new(
							gameshow_state_clone.id,
							input.value()
						);

						gameshow_state_clone.set(edited_gameshow);
					}
				})}
				class="border rounded px-4 py-2 mr-2"
			/>

			<button
				onclick=
				{
					gameshow_system.create_gameshow.clone()
				}
				class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
				{ 
					"Create Game Show Season"
				}
			</button>

			<ul class="list-disc pl-5">
			{
				for (*gameshow_system.gameshows).iter().map(|gameshow|
				{
					let gameshow_id = gameshow.id;
					html!
					{
						<li class="mb-2">
						<span class="font-semibold text-[#4a90e2]">{ format!("ID: {}, Name: {}", gameshow.id, gameshow.name) }</span>

						<button
							onclick={gameshow_system.delete_gameshow.clone().reform(move |_| gameshow_id)}
							class="ml-4 bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded">
							{ "Delete" }
						</button>
					</li>
					}
			})}
			</ul>

			<div class="mb-4">
				<select onchange={on_select_showseason}>
					<option value="" disabled=true selected=true>{"Select a show season"}</option>
					{
						// 2. Iterate over the vector and map to options
						gameshow_system.gameshows.iter().map(|show|
						{
							html!
							{
								<option key={show.id} value={show.id.to_string()}>
								{
									&show.name
								}
								</option>
							}
						}).collect::<Html>()
					}
				</select>
			</div>

			<input placeholder="Full Name of Contestant"
				value={contestant_system.contestant_state.name.clone()}
				oninput={Callback::from(
				{
					let contestant_state_clone = contestant_system.contestant_state.clone();
					move |e: InputEvent|
					{
						let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();

						let edited_contestant = ContestantState::new(
							contestant_state_clone.id,
							input.value(),
							None
						);

						contestant_state_clone.set(edited_contestant);
					}
				})}
				class="border rounded px-4 py-2 mr-2"
			/>

			<button
				onclick=
				{
					contestant_system.create_contestant.clone()
				}
				class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
				{
					"Create Contestant"
				}
			</button>

			<button
				onclick=
				{
					contestant_system.select_contestant.clone()
				}
				class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
				{
					"Select Contestant"
				}
			</button>

			<button
				onclick = 
				{
					contestant_system.enroll_contestant_onto_show.clone()
					.reform(
					{
						//logger::logger::log("Enrol Req con.id>>>".to_string() + contestant_system.contestant_state.id.unwrap_or(-1).to_string().as_str());

						let contestant_state_to_send = ContestantState::new(
							contestant_system.contestant_state.id,
							contestant_system.contestant_state.name.clone(),
							gameshow_system.gameshow_state.id,
						);

						//logger::logger::log("Enrol Req 222 (con.id)>>>".to_string() + contestant_system.contestant_state.id.unwrap_or(-1).to_string().as_str());

						move |_| contestant_state_to_send.clone()
					})				
				}

				class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded">
				{
					"Enroll Contestant onto Show"
				}
			</button>

			<button
				onclick={contestant_system.delete_contestant.clone().reform(
				{
					let contestant_state_clone = contestant_system.contestant_state.clone();
					move |_| contestant_state_clone.name.clone()
				})}

				class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded">
				{
					"Delete Contestant"
				}
			</button>

		</>
	}
}

fn build_user_management(
	message: &UseStateHandle<String>,
	user_system : &UserSystem
) -> Html
{
	html!
	{
		<>

			<div class="mb-4">
				<input placeholder="Name"
					value={user_system.user_state.name.clone()}
					oninput={Callback::from(
					{
						let user_state_clone = user_system.user_state.clone();
						move |e: InputEvent|
						{
							let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();

							let edited_user = UserState::new(
								user_state_clone.id,
								input.value(),
								user_state_clone.email.clone(),
								user_state_clone.account_type.clone()
								);

							user_state_clone.set(edited_user);
						}
					})}
					class="border rounded px-4 py-2 mr-2"/>
				<input placeholder="Email"
					value={user_system.user_state.email.clone()}
					oninput={Callback::from(
					{
						let user_state_clone = user_system.user_state.clone();
						move |e: InputEvent|
						{
							let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();

							let edited_user = UserState::new(
								user_state_clone.id,
								user_state_clone.name.clone(),
								input.value(),
								user_state_clone.account_type.clone()
								);

							user_state_clone.set(edited_user);
						}
					})}
					class="border rounded px-4 py-2 mr-2"/>

				<button
					onclick=
					{
						if user_system.user_state.id.is_some()
						{
							user_system.update_user.clone()
						}
						else
						{
							user_system.create_user.clone()
						}
					}
					class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
					{ 
						if user_system.user_state.id.is_some()
						{
							"Update User"
						}
						else
						{ 
							"Create User"
						}
					}
				</button>

				if !message.is_empty()
				{
					<p class="text-green-500 mt-2">{ &**message }</p>
				}
			</div>

			<button
				onclick={user_system.get_users.reform(|_| ())}
				class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded mb-4">
				{ "Fetch Users" }
			</button>

			<h2 class="text-2xl font-bold text-[#FF8C00] mb-2">{ "User List" }</h2>

			<ul class="list-disc pl-5">
			{
				for (*user_system.users).iter().map(|user|
				{
					let user_id = user.id;
					html!
					{
						<li class="mb-2">
						<span class="font-semibold text-[#4a90e2]">{ format!("ID: {}, Name: {}, Email: {} AccountType: {} ", user.id, user.name, user.email, user.account_type) }</span>
						<button
							onclick={user_system.edit_user.clone().reform(move |_| user_id)}
							class="ml-4 bg-yellow-500 hover:bg-yellow-700 text-white font-bold py-1 px-2 rounded">
							{ "Edit" }
						</button>
						<button
							onclick={user_system.delete_user.clone().reform(move |_| user_id)}
							class="ml-4 bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded">
							{ "Delete" }
						</button>
					</li>
				}
			})}
			</ul>
		</>
	}
}