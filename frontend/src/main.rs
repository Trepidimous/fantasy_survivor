use yew::prelude::*;
use yew_router::prelude::*;

mod web_server;
mod users;
mod gameshows;
mod contestants;
mod logger;
mod gamemaster_portal;
mod player_portal;

use crate::users::users::*;
use crate::gameshows::gameshows::*;
use crate::contestants::contestants::*;

fn main()
{
	yew::Renderer::<App>::new().render();
}

#[derive(Clone, Routable, PartialEq)]
enum Route
{
	#[at("/gamemaster-portal")]
	GameMasterPortal,
	#[at("/player-portal")]
	PlayerPortal,
}

#[function_component(App)]
fn app() -> Html
{
	let message: UseStateHandle<String> = use_state(|| "".to_string());
	let user_system: UserSystem = users::users::use_compile_user_system(message.clone());
	let gameshow_system: GameShowSystem = gameshows::gameshows::use_compile_gameshow_system(message.clone());
	let contestant_system: ContestantSystem = contestants::contestants::use_compile_contestant_system(message.clone());

	let portal_router = 
	{
		let message: UseStateHandle<String> = message.clone();
		let user_system: UserSystem = user_system.clone();
		let gameshow_system: GameShowSystem = gameshow_system.clone();
		let contestant_system: ContestantSystem = contestant_system.clone();

		let dragged_index = use_state(|| None::<usize>);

		let ranked_contestants = use_state(|| Vec::<ContestantState>::new());

		// Effect: When the system fetches new contestants, update our local ranking list
		{
			let ranked_contestants = ranked_contestants.clone();
			let system_contestants = contestant_system.contestants_on_show.clone();
			use_effect_with(system_contestants, move |sys_c| {
					ranked_contestants.set((**sys_c).clone());
			});
		}

		move | routes: Route | match routes
		{
			Route::GameMasterPortal => gamemaster_portal::gamemaster_portal::build_gamemaster_portal_page(&message, &user_system, &gameshow_system, &contestant_system),
			Route::PlayerPortal => player_portal::player_portal::build_player_portal_page(&message, &user_system, &gameshow_system, &contestant_system, &dragged_index, &ranked_contestants)
		}
	};

	html!
	{
		<BrowserRouter>
			<Switch<Route> render={portal_router} />
		</BrowserRouter>
	}

}
