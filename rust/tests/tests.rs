#[cfg(test)]
mod test {
	extern crate envfile;
	extern crate apiclient;
	use reqwest;
	use std::env;
	use envfile::EnvFile;
	use std::path::Path;

	// Load API key.
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

	// Test the get test customers call.
	#[tokio::test]
	async fn test_get_customers()
	{
		let api_key = load_key();
		let http_client = reqwest::Client::new();

		let test_customers = apiclient::get_test_customers(&http_client, &api_key).await.unwrap();

		// Test if the number of test customers is as expected.
		assert_eq!(test_customers.len(), 10);
	}

	// Test the get currency conversion call.
	#[tokio::test]
	async fn test_get_currency_conversions()
	{
		let api_key = load_key();
		let http_client = reqwest::Client::new();

		let currency_conversions = apiclient::get_currency_conversions(&http_client, &api_key, "EUR").await.unwrap();

		assert_eq!(currency_conversions.len(), 45);
		
		for currency in currency_conversions.iter()
		{
			assert_ne!(currency.baseCurrency, "", "Unexpected baseCurrency.");
			assert_ne!(currency.quoteCurrency, "", "Unexpected quoteCurrency.");
			assert_ne!(currency.country, "", "Unexpected country.");
			assert_ne!(currency.updatedDate, "", "Unexpected updatedDate.");
			assert_ne!(currency.amount, f32::MIN, "Unexpected amount.");
			assert_ne!(currency.buyRateTransfer, f32::MIN, "Unexpected buyRateTransfer.");
			assert_ne!(currency.sellRateTransfer, f32::MIN, "Unexpected sellRateTransfer.");
			assert_ne!(currency.midRate, f32::MIN, "Unexpected midRate.");
			assert_ne!(currency.changeInMidRate, f32::MIN, "Unexpected changeInMidRate.");
			assert_ne!(currency.previousMidRate, f32::MIN, "Unexpected previousMidRate.");
			assert_ne!(currency.buyRateCash, f32::MIN, "Unexpected buyRateCash.");
			assert_ne!(currency.sellRateCash, f32::MIN, "Unexpected sellRateCash.");
		}
	}

	// Test the get currency conversion call.
	#[tokio::test]
	async fn test_get_currency_conversion()
	{
		let api_key = load_key();
		let http_client = reqwest::Client::new();

		let currency_conversion = apiclient::get_currency_conversion(&http_client, &api_key, "NOK", "EUR").await.unwrap();

		assert_eq!(currency_conversion.baseCurrency, "EUR");
		assert_eq!(currency_conversion.quoteCurrency, "NOK");
		assert_eq!(currency_conversion.country, "EU");
		assert_ne!(currency_conversion.updatedDate, "");
		assert_ne!(currency_conversion.amount, f32::MIN);
		assert_ne!(currency_conversion.buyRateTransfer, f32::MIN);
		assert_ne!(currency_conversion.sellRateTransfer, f32::MIN);
		assert_ne!(currency_conversion.midRate, f32::MIN);
		assert_ne!(currency_conversion.changeInMidRate, f32::MIN);
		assert_ne!(currency_conversion.previousMidRate, f32::MIN);
		assert_ne!(currency_conversion.buyRateCash, f32::MIN);
		assert_ne!(currency_conversion.sellRateCash, f32::MIN);
	}
}
