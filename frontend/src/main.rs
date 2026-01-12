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
	let user_state: UseStateHandle<(String, String, String, Option<i32>)> = use_state(|| ("".to_string(), "".to_string(), "".to_string(), None as Option<i32>));
	let message: UseStateHandle<String> = use_state(|| "".to_string());
	let users: UseStateHandle<Vec<User>> = use_state(Vec::new);

	let get_users: Callback<()> = get_users(&users, &message);

	let create_user: yew::Callback<yew::MouseEvent> = create_user(&user_state, &message, get_users.clone());

	let update_user: Callback<MouseEvent> = update_user(&user_state, &message, get_users.clone());

	let delete_user: Callback<i32> = delete_user(&message, get_users.clone());

	let edit_user: Callback<i32> = edit_user(&user_state, &users);

	print_html(&user_state, &message, &users, get_users.clone(), create_user.clone(), update_user.clone(), delete_user.clone(), edit_user.clone())
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
	let users = users.clone();
	let message = message.clone();
	Callback::from(move |_|
	{
		let users = users.clone();
		let message = message.clone();
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

fn create_user(user_state: &UseStateHandle<(String, String, String, Option<i32>)>, 
	message: &UseStateHandle<String>,
	get_users: Callback<()>) -> yew::Callback<yew::MouseEvent>
{
	return
	{
		let user_state = user_state.clone();
		let message = message.clone();
		let get_users = get_users.clone();
		Callback::from(move |_|
		{
			let (name, email, _account_type, _) = (*user_state).clone();
			let user_state = user_state.clone();
			let message = message.clone();
			let get_users = get_users.clone();
			let account_type_in = "Developer"; // GameMaster, Admin, Player

			spawn_local(async move
			{
				let user_data = serde_json::json!({ "name": name, "email": email, "account_type" : account_type_in });
				let response = Request::post("http://127.0.0.1:8000/api/users")
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

				user_state.set(("".to_string(), "".to_string(), "".to_string(), None));
			});
		})
	};
}

fn update_user(user_state: &UseStateHandle<(String, String, String, Option<i32>)>,
	message: &UseStateHandle<String>,
	get_users: Callback<()>) -> Callback<MouseEvent>
{
	return 
	{
		let user_state = user_state.clone();
		let message = message.clone();
		let get_users = get_users.clone();

		Callback::from(move |_|
		{
			let (name, email, _account_type, editing_user_id ) = (*user_state).clone();
			let user_state = user_state.clone();
			let message = message.clone();
			let get_users = get_users.clone();

			if let Some(id) = editing_user_id
			{
				spawn_local(async move
				{
					let response = Request::put(&format!("http://127.0.0.1:8000/api/users/{}", id))
						.header("Content-Type", "application/json")
						.body( serde_json::to_string(&(id, name.as_str(), email.as_str(), _account_type.as_str() )).unwrap())
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

					user_state.set(("".to_string(), "".to_string(), "".to_string(), None));
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
		let message = message.clone();
		let get_users = get_users.clone();

		Callback::from(move |id: i32|
		{
			let message = message.clone();
			let get_users = get_users.clone();

			spawn_local(async move
			{
				let response = Request::delete(
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

fn edit_user(user_state : &UseStateHandle<(String, String, String, Option<i32>)>, users : &UseStateHandle<Vec<User>>) -> Callback<i32>
{
	return
	{
		let user_state = user_state.clone();
		let users = users.clone();

		Callback::from(move |id: i32|
		{
			if let Some(user) = users.iter().find(|u| u.id == id)
			{
				user_state.set((user.name.clone(), user.email.clone(), user.account_type.clone(), Some(id)));
			}
		})
	};
}

fn print_html(user_state: &UseStateHandle<(String, String, String, Option<i32>)>,
	message: &UseStateHandle<String>,
	users: &UseStateHandle<Vec<User>>,
	get_users: Callback<()>,
	create_user: yew::Callback<yew::MouseEvent>,
	update_user: Callback<MouseEvent>,
	delete_user: Callback<i32>,
	edit_user: Callback<i32>) -> Html
{
	html!
	{
		<body class="bg-[#121212]  min-h-screen">
			<div class="container mx-auto p-4">
				<h1 class="text-4xl font-bold text-[#FF8C00] mb-4">{ "Game Master Portal" }</h1>
					<div class="mb-4">
						<input placeholder="Name"
							value={user_state.0.clone()}
							oninput={Callback::from(
							{
								let user_state = user_state.clone();
								move |e: InputEvent|
								{
									let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
									user_state.set((input.value(), user_state.1.clone(), user_state.2.clone(), user_state.3));
								}
							})}
							class="border rounded px-4 py-2 mr-2"/>
						<input placeholder="Email"
							value={user_state.1.clone()}
							oninput={Callback::from(
							{
								let user_state = user_state.clone();
								move |e: InputEvent|
								{
									let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
									user_state.set((user_state.0.clone(), input.value(), user_state.2.clone(), user_state.3));
								}
							})}
							class="border rounded px-4 py-2 mr-2"/>

						<button
							onclick=
							{
								if user_state.3.is_some()
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
								if user_state.3.is_some()
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


