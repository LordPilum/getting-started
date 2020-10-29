use reqwest;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::vec::Vec;
use serde::{Deserialize, Serialize};
use std::fmt;

// Defines the data structure for the currences APIs.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CurrencyConversions
{
    pub baseCurrency: String,
    pub quoteCurrency: String,
    pub country: String,
    pub updatedDate: String,
    pub amount: f32,
    pub buyRateTransfer: f32,
    pub sellRateTransfer: f32,
    pub midRate: f32,
    pub changeInMidRate: f32,
    pub previousMidRate: f32,
    pub buyRateCash: f32,
    pub sellRateCash: f32
}

// Implements output formatting for the data structure.
impl fmt::Debug for CurrencyConversions
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f,
            "{{\n\tbaseCurrency: {},\n\tquoteCurrency: {}\n\tcountry: {}\n\tupdatedDate: {}\n\tamount: {}\n\tbuyRateTransfer: {}\n\tsellRateTransfer: {}\n\tmidRate: {}\n\tchangeInMidRate: {}\n\tpreviousMidRate: {}\n\tbuyRateCash: {}\n\tsellRateCash: {}\n}}",
            self.baseCurrency,
            self.quoteCurrency,
            self.country,
            self.updatedDate,
            self.amount,
            self.buyRateTransfer,
            self.sellRateTransfer,
            self.midRate,
            self.changeInMidRate,
            self.previousMidRate,
            self.buyRateCash,
            self.sellRateCash
        )
    }
}


// Function to make asynchronous GET requests at the given path.
// The host is currently hardcoded.
async fn make_request(client: &reqwest::Client, path: &str, api_key: &str) -> Result<reqwest::Response, reqwest::Error>
{
    // Get the current date.
    let now: DateTime<Utc> = Utc::now();
    let host = "https://developer-api-testmode.dnb.no";
    let url = [host, path].concat();

    client
        .get(&url)
        .header("Accept", "application/json")
        .header("Content-type", "application/json")
        .header("x-api-key", api_key)
        .header("x-amz-date", now.format("%Y%m%dT%H%M%SZ").to_string())
        .send().await
}

// Gets a list of test customers.
// The function returns a hash map for easy output formatting.
pub async fn get_test_customers(client: &reqwest::Client, api_key: &str) -> Result<Vec<HashMap<String, String>>, reqwest::Error>
{
    let path = "/test-customers/v0";

    let result = make_request(client, path, api_key).await?;

    result.json::<Vec<HashMap<String, String>>>().await
}

// Gets a list of currency conversions for the quoted currency.
// The function returns a list of currency conversions.
pub async fn get_currency_conversions(client: &reqwest::Client, api_key: &str, quote_currency: &str) -> Result<Vec<CurrencyConversions>, reqwest::Error>
{
    let path = ["/currencies/v1/convert/", quote_currency].concat();

    let result = make_request(client, &path, api_key).await?;

    result.json::<Vec<CurrencyConversions>>().await
}

// Gets a specified currency conversion.
// The function returns a single currency conversion struct.
pub async fn get_currency_conversion(client: &reqwest::Client, api_key: &str, quote_currency: &str, base_currency: &str) -> Result<CurrencyConversions, reqwest::Error>
{
    let path = ["/currencies/v1/", base_currency, "/convert/", quote_currency].concat();

    let result = make_request(client, &path, api_key).await?;

    result.json::<CurrencyConversions>().await
}
