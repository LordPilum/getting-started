extern crate envfile;
mod apiclient;
use reqwest;
use std::env;
use envfile::EnvFile;
use std::path::Path;

fn load_key() -> String
{
	// Read the API key from the environment variable, if set.
	let api_key = match env::var("API_KEY")
	{
		Ok(key) => key,
		Err(_) =>
		{
			// Read the API key from the .env file.
			let envfile = EnvFile::new(&Path::new(".env")).unwrap();
			let key = envfile.get("API_KEY").unwrap();
			key.to_string()
		}
	};

	return api_key;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
	let api_key = load_key();

	// Create a new HTTP client.
	// Reqwest keeps an internal connection pool, so a client should only be created once and re-used.
	let http_client = reqwest::Client::new();
	
	// Test customers
	// Get test customers.
	let test_customers = apiclient::get_test_customers(&http_client, &api_key).await?;

	// Print the test customers.
	println!("{:#?}", test_customers);

	// Currency conversions
	// Get a list of currency conversions.
	let currency_conversions_list = apiclient::get_currency_conversions(&http_client, &api_key, "EUR").await?;

	// Print the currency conversions.
	println!("{:#?}", currency_conversions_list);

	// Gets a single currency conversion.
	let currency_conversion = apiclient::get_currency_conversion(&http_client, &api_key, "NOK", "EUR").await?;

	// Print the currency conversion.
	println!("{:#?}", currency_conversion);

	Ok(())
}
