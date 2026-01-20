
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserState
{
	pub name: String,
	pub email: String,
	pub account_type: String,
	pub id: Option<i32>,
}

impl UserState
{
	pub fn from_default() -> Self
	{
		UserState
		{
			name: "".to_string(),
			email: "".to_string(),
			account_type: "".to_string(),
			id: None,
		}
	}

	pub fn new(id_in : Option<i32>, name_in : String, email_in : String, account_type_in : String) -> Self
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
