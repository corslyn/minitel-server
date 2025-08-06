use dotenv::dotenv;
use reqwest::blocking::Client;

struct Meteo {
    ville: String,
    temperature: f32,
    description: String,
}

pub fn main_meteo(ville: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY not set in .env file");
    let json_response = fetch_meteo_data(&api_key, ville).expect("Failed to fetch meteo data");
    println!("Météo pour {}: {}", ville, json_response);
    parse_json(&json_response)?;
    Ok(())
}

fn fetch_meteo_data(api_key: &str, ville: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&lang=fr&units=metric",
        ville, api_key
    );

    let response = client.get(&url).send()?;

    if response.status().is_success() {
        let json = response.text()?;
        Ok(json)
    } else {
        Err(format!("Failed to fetch data: {}", response.status()).into())
    }
}

fn parse_json(json: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data: serde_json::Value = serde_json::from_str(json)?;

    Ok(())
}
