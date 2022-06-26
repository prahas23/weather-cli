mod forecast;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let city = std::env::args().nth(1).expect("no city given");
    let country_code = std::env::args().nth(2).expect("no country code given");
    let forecast = forecast::Forecast::get(city, country_code).await?;
    let temp = kelvin_to_celcius(forecast.main.temp);
    println!("Description: {}, Temperature: {:.2}", forecast.weather.details.description, temp);
    Ok(())
}

fn kelvin_to_celcius(kelvin:f64) -> f64 {
    kelvin - 273.15
}