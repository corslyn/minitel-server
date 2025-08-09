use dotenv::dotenv;
use reqwest::blocking::Client;

pub fn main_meteo(ville: &str) -> Result<(String, String, f64, i64), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY not set in .env file");
    let json_response = fetch_meteo_data(&api_key, ville).expect("Failed to fetch meteo data");
    let data = parse_json(&json_response)?;

    Ok(data)
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

fn parse_json(json: &str) -> Result<(String, String, f64, i64), Box<dyn std::error::Error>> {
    let data: serde_json::Value = serde_json::from_str(json)?;
    let weather_array = data["weather"].as_array().ok_or("No weather data")?;
    let obj_weather = &weather_array[0];

    let id = obj_weather["id"].as_i64().unwrap_or_default();
    let desc = obj_weather["description"].as_str().unwrap_or("N/A");

    // Extract main object
    let main = &data["main"];
    let temp = main["temp"].as_f64().unwrap_or_default();
    let pression = main["pressure"].as_i64().unwrap_or_default();

    let ville = data["name"].as_str().unwrap_or("N/A");
    log::info!("Ville: {}", ville);
    log::info!("Description: {}", desc);
    log::info!("Température: {}°C", temp);
    log::info!("Pression: {} hPa", pression);
    Ok((ville.to_string(), desc.to_string(), temp, pression))
}
