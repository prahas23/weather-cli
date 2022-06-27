mod forecast;
mod modify_forecast;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Arguments passed while running the program
    let city = std::env::args().nth(1).expect("no city given");
    let country_code = std::env::args().nth(2).expect("no country code given");
    
    // Gets the forecast from the forecast module
    let forecast = forecast::Forecast::get(city, country_code).await?;
    
    // Modifies forecast based on the ID
    modify_forecast::modify(forecast);
    
    Ok(())
}
