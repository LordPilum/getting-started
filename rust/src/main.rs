mod apiclient;
use reqwest;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
	// Read the API key from the environment variable.
	let api_key = env::var("API_KEY").expect("API Key must be set");

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
