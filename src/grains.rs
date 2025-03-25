use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug)]
pub struct GrainData {
    pub name: String,
    pub current_price: f64, // Today's closing price
    pub daily_change: f64,  // Difference between today's and yesterday's close
}

// Structures to parse Yahoo Finance JSON response.
#[derive(Deserialize, Debug)]
struct ApiResponse {
    chart: Chart,
}

#[derive(Deserialize, Debug)]
struct Chart {
    result: Vec<ChartResult>,
}

#[derive(Deserialize, Debug)]
struct ChartResult {
    indicators: Indicators,
}

#[derive(Deserialize, Debug)]
struct Indicators {
    quote: Vec<Quote>,
}

#[derive(Deserialize, Debug)]
struct Quote {
    close: Option<Vec<Option<f64>>>,
}

/// Fetches futures data for a given grain symbol from Yahoo Finance.
///
/// It calls the Yahoo Finance endpoint for a given symbol with `range=5d` and `interval=1d`
/// so that we have multiple days of data. We then filter out null values from the
/// close prices and use the last two valid data points to compute the daily change.
///
/// Example futures symbols (Yahoo Finance):
///   - Wheat: "ZW=F"
///   - Corn:  "ZC=F"
///   - Rice:  "ZR=F"
///   - Barley: "ZB=F"
///   - Oats:  "ZO=F"
///   - Rye:   "ZRY=F" (placeholder)
///   - Millet: "ZMIL=F" (placeholder)
///   - Sorghum: "ZSOR=F" (placeholder)
///   - Quinoa:  "ZQUI=F" (placeholder)
async fn fetch_grain_price(
    symbol: &str,
    display_name: &str,
    client: Client,
) -> Result<GrainData, Box<dyn Error + Send + Sync + 'static>> {
    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}?range=5d&interval=1d",
        symbol
    );
    let resp = client
        .get(&url)
        .header(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (compatible; GrainFetch/0.1)",
        )
        .send()
        .await?;
    if !resp.status().is_success() {
        return Err(format!("Yahoo Finance API returned status {}", resp.status()).into());
    }
    let json: ApiResponse = resp.json().await?;
    let closes_opt = json
        .chart
        .result
        .get(0)
        .and_then(|result| result.indicators.quote.get(0))
        .and_then(|quote| quote.close.as_ref())
        .ok_or("Unable to parse close prices")?;

    // Filter out None values.
    let valid_closes: Vec<f64> = closes_opt.iter().filter_map(|opt| *opt).collect();
    if valid_closes.len() < 2 {
        return Err("Not enough data points returned".into());
    }

    // Use the last two valid data points.
    let yesterday_price = valid_closes[valid_closes.len() - 2];
    let today_price = valid_closes[valid_closes.len() - 1];
    let daily_change = today_price - yesterday_price;

    Ok(GrainData {
        name: display_name.to_string(),
        current_price: today_price,
        daily_change,
    })
}

/// Returns a vector of GrainData for common grains by concurrently fetching data
/// from Yahoo Finance. If fetching data for a grain fails, that grain is skipped.
pub async fn get_common_grains_data()
-> Result<Vec<GrainData>, Box<dyn Error + Send + Sync + 'static>> {
    // List of grain names and their Yahoo Finance tickers.
    let grains = vec![
        ("Wheat", "ZW=F"),
        ("Corn", "ZC=F"),
        ("Rice", "ZR=F"),
        ("Barley", "ZB=F"),
        ("Oats", "ZO=F"),
    ];

    let client = Client::new();
    let mut results = Vec::new();

    for (name, symbol) in grains {
        let client_clone = client.clone();
        match fetch_grain_price(symbol, name, client_clone).await {
            Ok(data) => results.push(data),
            Err(e) => eprintln!("Error fetching {}: {}", name, e),
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_fetch_grain_price() {
        let client = Client::new();
        let result = fetch_grain_price("ZW=F", "Wheat", client).await;
        match result {
            Ok(grain) => {
                println!("Fetched grain data: {:?}", grain);
                assert!(grain.current_price > 0.0);
            }
            Err(e) => {
                panic!("Error fetching grain price: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_common_grains_data() {
        let grains_data = get_common_grains_data().await;
        match grains_data {
            Ok(data) => {
                println!("Fetched common grains data: {:?}", data);
                assert!(!data.is_empty());
                for grain in data {
                    assert!(grain.current_price > 0.0);
                }
            }
            Err(e) => {
                panic!("Error fetching common grains data: {}", e);
            }
        }
    }
}
