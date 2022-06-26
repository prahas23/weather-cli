use dotenv::dotenv;
use reqwest::Url;
use std::env;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    pub weather: Weather,
    pub main: Temperature,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub pressure: i32,
    pub humidity: i32,
}

impl Forecast {
    pub async fn get(city: String, country_code: String) -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        let api_key = env::var("WEATHER_API_KEY")?;
        let url = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}", city, country_code, api_key);
        let url = Url::parse(&*url)?;

        let resp = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;
        Ok(resp)
    }
}
