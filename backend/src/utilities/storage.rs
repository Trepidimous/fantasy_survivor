
use tokio_postgres::{ Client, NoTls };

pub struct StorageConnector
{
	pub storage : Client,
}

impl StorageConnector
{
	pub async fn establish_connection_to_storage() -> Self
	{
		let (new_connection, connection) = tokio_postgres
			::connect("host=localhost user=postgres password=postgres dbname=postgres", NoTls).await
			.expect("Failed to connect to Postgres");

		tokio::spawn(async move
		{
			if let Err(e) = connection.await
			{
				eprintln!("Failed to connect to Postgres: {}", e);
			}
		});

		let storage_connector : StorageConnector = StorageConnector
		{
			storage: new_connection
		};

		return storage_connector;
	}
}