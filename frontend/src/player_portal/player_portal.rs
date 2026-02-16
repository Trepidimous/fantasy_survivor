use yew::prelude::*;

use crate::users::users::UserState;
use crate::users::users::*;
use crate::gameshows::gameshows::*;
use crate::contestants::contestants::*;
use crate::logger;


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

			</div>
		</body>
	}
}
