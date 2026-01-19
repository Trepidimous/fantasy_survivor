use yew::prelude::*;
use serde::{ Deserialize, Serialize };
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;

fn main()
{
	yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html
{
	let user_state: UseStateHandle<UserState> = use_state(|| UserState::from_default());
	let message: UseStateHandle<String> = use_state(|| "".to_string());
	let users: UseStateHandle<Vec<User>> = use_state(Vec::new);

	let gameshow_state : UseStateHandle<GameShowState> = use_state(|| GameShowState { name: "".to_string(), id: None });
	let gameshows: UseStateHandle<Vec<GameShow>> = use_state(Vec::new);
	let get_gameshows: Callback<()> = get_gameshows(&gameshows, &message);
	let create_gameshow: yew::Callback<yew::MouseEvent> = create_gameshow(&gameshow_state, &message, get_gameshows.clone());

	let get_users: Callback<()> = get_users(&users, &message);
	let create_user: yew::Callback<yew::MouseEvent> = create_user(&user_state, &message, get_users.clone());
	let update_user: Callback<MouseEvent> = update_user(&user_state, &message, get_users.clone());
	let delete_user: Callback<i32> = delete_user(&message, get_users.clone());
	let edit_user: Callback<i32> = edit_user(&user_state, &users);

	print_html(&user_state, &message, &users, get_users, create_user, update_user, delete_user, edit_user, &gameshow_state, &gameshows, get_gameshows, create_gameshow)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct UserState
{
	name: String,
	email: String,
	account_type: String,
	id: Option<i32>,
}

impl UserState
{
	fn from_default() -> Self
	{
		UserState
		{
			name: "".to_string(),
			email: "".to_string(),
			account_type: "".to_string(),
			id: None,
		}
	}

	fn new(id_in : Option<i32>, name_in : String, email_in : String, account_type_in : String) -> Self
	{
		UserState
		{
			name : name_in,
			email : email_in,
			account_type : account_type_in,
			id : id_in,
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct User
{
	id: i32,
	name: String,
	email: String,
	account_type: String,
}

fn get_users(users: &UseStateHandle<Vec<User>>,
	message: &UseStateHandle<String>) -> Callback<()>
{
	let users: UseStateHandle<Vec<User>> = users.clone();
	let message: UseStateHandle<String> = message.clone();
	Callback::from(move |_|
	{
		let users: UseStateHandle<Vec<User>> = users.clone();
		let message: UseStateHandle<String> = message.clone();
		spawn_local(async move
		{
			match Request::get("http://127.0.0.1:8000/api/users").send().await
			{
				Ok(resp) if resp.ok() =>
				{
					let fetched_users: Vec<User> = resp.json().await.unwrap_or_default();
					users.set(fetched_users);
				}

				_ => message.set("Failed to fetch users".into()),
			}
		});
	})
}

fn create_user(user_state: &UseStateHandle<UserState>, 
	message: &UseStateHandle<String>,
	get_users: Callback<()>) -> yew::Callback<yew::MouseEvent>
{
	return
	{
		let user_state: UseStateHandle<UserState> = user_state.clone();
		let message: UseStateHandle<String> = message.clone();
		let get_users: Callback<()> = get_users.clone();
		Callback::from(move |_|
		{
			let user_state: UseStateHandle<UserState> = user_state.clone();
			let message: UseStateHandle<String> = message.clone();
			let get_users: Callback<()> = get_users.clone();
			let account_type_in: &str = "Developer"; // GameMaster, Admin, Player

			spawn_local(async move
			{
				let user_data: serde_json::Value = serde_json::json!({ "name": user_state.name, "email": user_state.email, "account_type" : account_type_in });
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::post("http://127.0.0.1:8000/api/users")
					.header("Content-Type", "application/json")
					.body(user_data.to_string())
					.send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						message.set("User created successfully".into());
						get_users.emit(());
					}

					_ => message.set("Failed to create user".into()),
				}

				user_state.set(UserState::from_default());
			});
		})
	};
}

fn update_user(user_state: &UseStateHandle<UserState>,
	message: &UseStateHandle<String>,
	get_users: Callback<()>) -> Callback<MouseEvent>
{
	return 
	{
		let user_state: UseStateHandle<UserState> = user_state.clone();
		let message: UseStateHandle<String> = message.clone();
		let get_users: Callback<()> = get_users.clone();

		Callback::from(move |_|
		{
			let editing_user_id: Option<i32> = user_state.id;
			let user_state: UseStateHandle<UserState> = user_state.clone();
			let message: UseStateHandle<String> = message.clone();
			let get_users: Callback<()> = get_users.clone();

			if let Some(id) = editing_user_id
			{
				spawn_local(async move
				{
					let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::put(&format!("http://127.0.0.1:8000/api/users/{}", id))
						.header("Content-Type", "application/json")
						.body( serde_json::to_string(&(id, user_state.name.as_str(), user_state.email.as_str(), user_state.account_type.as_str() )).unwrap())
						.send().await;

					match response
					{
						Ok(resp) if resp.ok() =>
						{
							message.set("User updated successfully".into());
							get_users.emit(());
						}

						_ => message.set("Failed to update user".into()),
					}

					user_state.set(UserState::from_default());
				});
			}
		})
	};
}

fn delete_user(message: &UseStateHandle<String>,
	get_users: Callback<()>) -> Callback<i32>
{
	return
	{
		let message: UseStateHandle<String> = message.clone();
		let get_users: Callback<()> = get_users.clone();

		Callback::from(move |id: i32|
		{
			let message: UseStateHandle<String> = message.clone();
			let get_users: Callback<()> = get_users.clone();

			spawn_local(async move
			{
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::delete(
					&format!("http://127.0.0.1:8000/api/users/{}", id)
				).send().await;

				match response
				{
					Ok(resp) if resp.ok() =>
					{
						message.set("User deleted successfully".into());
						get_users.emit(());
					}

					_ => message.set("Failed to delete user".into()),
				}
			});
		})
	};
}

fn edit_user(user_state : &UseStateHandle<UserState>, users : &UseStateHandle<Vec<User>>) -> Callback<i32>
{
	return
	{
		let user_state_handle: UseStateHandle<UserState> = user_state.clone();
		let users: UseStateHandle<Vec<User>> = users.clone();

		Callback::from(move |id: i32|
		{
			if let Some(user) = users.iter().find(|u: &&User| u.id == id)
			{
				let edited_user: UserState = UserState::new(
					Some(user.id),
					user.name.clone(),
					user.email.clone(),
					user.account_type.clone());

				user_state_handle.set(edited_user);
			}
		})
	};
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GameShow
{
	id: i32,
	name: String,
}

struct GameShowState
{
	name: String,
	id: Option<i32>,
}

impl GameShowState
{
	fn new(id_in : Option<i32>, name_in : String) -> Self
	{
		GameShowState
		{
			name : name_in,
			id : id_in
		}
	}
}

fn get_gameshows(gameshows: &UseStateHandle<Vec<GameShow>>,
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
			match Request::get("http://127.0.0.1:8000/api/gameshows").send().await
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

fn create_gameshow(gameshow_state: &UseStateHandle<GameShowState>,
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
				let response: Result<gloo::net::http::Response, gloo::net::Error> = Request::post("http://127.0.0.1:8000/api/gameshows")
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



fn print_html(user_state: &UseStateHandle<UserState>,
	message: &UseStateHandle<String>,
	users: &UseStateHandle<Vec<User>>,
	get_users: Callback<()>,
	create_user: yew::Callback<yew::MouseEvent>,
	update_user: Callback<MouseEvent>,
	delete_user: Callback<i32>,
	edit_user: Callback<i32>,
	gameshow_state : &UseStateHandle<GameShowState>,
	gameshows: &UseStateHandle<Vec<GameShow>>,
	get_gameshows: Callback<()>,
	create_gameshow: yew::Callback<yew::MouseEvent>
) -> Html
{
	html!
	{
		<body class="bg-[#121212]  min-h-screen">
			<div class="container mx-auto p-4">
				<h1 class="text-4xl font-bold text-[#FF8C00] mb-4">{ "Game Master Portal" }</h1>
					<button
						onclick={get_gameshows.reform(|_| ())}
						class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded mb-4">
						{ "Fetch Game Shows" }
					</button>

					<input placeholder="New Game Show Name"
						value={gameshow_state.name.clone()}
						oninput={Callback::from(
						{
							let gameshow_state_clone = gameshow_state.clone();
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
							create_gameshow.clone()
						}
						class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
						{ 
							"Create Game Show"
						}
					</button>

					<ul class="list-disc pl-5">
					{
						for (*gameshows).iter().map(|gameshow|
						{
							let gameshow_id = gameshow.id;
							html!
							{
								<li class="mb-2">
								<span class="font-semibold text-[#4a90e2]">{ format!("ID: {}, Name: {}", gameshow.id, gameshow.name) }</span>
								<button
									onclick={edit_user.clone().reform(move |_| gameshow_id)}
									class="ml-4 bg-yellow-500 hover:bg-yellow-700 text-white font-bold py-1 px-2 rounded">
									{ "Edit" }
								</button>
								<button
									onclick={delete_user.clone().reform(move |_| gameshow_id)}
									class="ml-4 bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded">
									{ "Delete" }
								</button>
							</li>
						}
					})}
					</ul>

					<div class="mb-4">

						<input placeholder="Name"
							value={user_state.name.clone()}
							oninput={Callback::from(
							{
								let user_state_clone = user_state.clone();
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
							value={user_state.email.clone()}
							oninput={Callback::from(
							{
								let user_state_clone = user_state.clone();
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
								if user_state.id.is_some()
								{
									update_user.clone()
								}
								else
								{
									create_user.clone()
								}
							}
							class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
							{ 
								if user_state.id.is_some()
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
						onclick={get_users.reform(|_| ())}
						class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded mb-4">
						{ "Fetch User List" }
					</button>

					<h2 class="text-2xl font-bold text-[#FF8C00] mb-2">{ "User List" }</h2>

					<ul class="list-disc pl-5">
					{
						for (*users).iter().map(|user|
						{
							let user_id = user.id;
							html!
							{
								<li class="mb-2">
								<span class="font-semibold text-[#4a90e2]">{ format!("ID: {}, Name: {}, Email: {} AccountType: {} ", user.id, user.name, user.email, user.account_type) }</span>
								<button
									onclick={edit_user.clone().reform(move |_| user_id)}
									class="ml-4 bg-yellow-500 hover:bg-yellow-700 text-white font-bold py-1 px-2 rounded">
									{ "Edit" }
								</button>
								<button
									onclick={delete_user.clone().reform(move |_| user_id)}
									class="ml-4 bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded">
									{ "Delete" }
								</button>
							</li>
						}
					})}
					</ul>
			</div>
		</body>
	}
}