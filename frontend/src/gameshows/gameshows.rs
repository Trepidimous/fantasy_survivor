use yew::prelude::*;
use serde::{ Deserialize, Serialize };
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;

use crate::web_server::PLATFORM_URL;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameShow
{
	pub id: i32,
	pub name: String,
}

pub struct GameShowState
{
	pub name: String,
	pub id: Option<i32>,
}

impl GameShowState
{
	pub fn new(id_in : Option<i32>, name_in : String) -> Self
	{
		GameShowState
		{
			name : name_in,
			id : id_in
		}
	}
}