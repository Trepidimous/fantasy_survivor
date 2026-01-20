

pub struct ContestantState
{
	pub name: String,
	pub id: Option<i32>,
}

impl ContestantState
{
	pub fn new(id_in : Option<i32>, name_in : String) -> Self
	{
		ContestantState
		{
			name : name_in,
			id : id_in
		}
	}
}