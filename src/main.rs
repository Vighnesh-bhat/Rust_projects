use std::fs::File;
use std::io::Write;
use std::time::Duration;
use clap::{App, Arg};
use tokio::time::Instant;
use reqwest;

#[derive(Debug, serde::Deserialize)]
struct PriceResponse {
    price: String,
}

impl PriceResponse {
    fn get_price_as_f64(&self) -> f64 {
        self.price.parse().unwrap_or(0.0)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("HTTPS Price Client")
        .version("1.0")
        .author("Your Name")
        .about("HTTPS client for fetching BTC/USDT prices")
        .arg(
            Arg::with_name("mode")
                .short("m")
                .long("mode")
                .value_name("MODE")
                .help("Sets the mode (cache or read)")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("times")
                .short("t")
                .long("times")
                .value_name("SECONDS")
                .help("Sets the duration in seconds")
                .required_if("mode", "cache")
                .takes_value(true),
        )
        .get_matches();

    let mode = matches.value_of("mode").unwrap();

    match mode {
        "cache" => {
            let duration_seconds: u64 = matches.value_of("times").unwrap().parse().unwrap();
            run_cache_mode(duration_seconds).await?;
        }
        "read" => {
            run_read_mode()?;
        }
        _ => {
            eprintln!("Invalid mode. Supported modes are 'cache' and 'read'");
        }
    }

    Ok(())
}

async fn run_cache_mode(duration_seconds: u64) -> Result<(), Box<dyn std::error::Error>> {
    let binance_api_url = "https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT";
    let start_time = Instant::now();
    let mut prices: Vec<f64> = Vec::new();

    while Instant::now() - start_time < Duration::from_secs(duration_seconds) {
        let response = reqwest::get(binance_api_url).await?;
        let body = response.text().await?;

        if let Ok(price) = serde_json::from_str::<PriceResponse>(&body) {
            println!("Current BTC price in USD: {}", price.price);
            prices.push(price.get_price_as_f64());
        }
    }

    
    let average_price = prices.iter().sum::<f64>() / prices.len() as f64;


    println!("Cache complete. The average USD price of BTC is: {}", average_price);

    
    save_to_file("cache_results.txt", average_price)?;

    Ok(())
}

fn run_read_mode() -> Result<(), Box<dyn std::error::Error>> {
    // Read data from the file
    let file_content = std::fs::read_to_string("cache_results.txt")?;
    println!("Read mode: {}", file_content);
    Ok(())
}

fn save_to_file(file_path: &str, average_price: f64) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    writeln!(file, "Average USD price of BTC: {}", average_price)?;
    Ok(())
}
