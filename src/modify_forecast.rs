use colored::Colorize;
use crate::forecast::Forecast;

pub fn modify(forecast: Forecast) {

    // Gets the ID of the weather
    let id = forecast.weather.details.id;

    // Gets the details of the weather
    let details = &forecast.weather.details.description;

    let changed_des;
    
    // Clouds
    if id > 800 {
        changed_des = details.red().bold();
        println!("\nDescription: {} ☁️", changed_des);
        print_result(forecast);
    }
    
    // Clear sky
    else if id == 800 {
        changed_des = details.green().bold();
        println!("\nDescription: {} ⛅", changed_des);
        print_result(forecast);
    }

    // Rain
    else if id >= 500 && id <= 531 {
        changed_des = details.blue().bold();
        println!("\nDescription: {} 🌧️", changed_des);
        print_result(forecast);
    }

    // Thunderstorm
    else if id >= 200 && id <= 232{
        changed_des = details.white().yellow();
        println!("\nDescription: {} 🌩️", changed_des);
        print_result(forecast);
    }
}

pub fn print_result(forecast: Forecast) {

    // Gets the temperature in kelvin
    let temp = forecast.main.temp;
    let temp_feels_like = forecast.main.feels_like;

    // Converts temperature from kelvin to celcius
    let temp_celcius = kelvin_to_celcius(temp);
    let temp_feels_like_celcius = kelvin_to_celcius(temp_feels_like);

    // Converts temperature from celcius to Fahrenheit
    let temp_fahr = celcius_to_fahrenheit(temp_celcius);
    let temp_feels_like_fahr = celcius_to_fahrenheit(temp_feels_like_celcius);

    println!("Temperature: {:.2} °C, {:.2} °F", temp_celcius, temp_fahr);
    println!("Feels like: {:.2} °C, {:.2} °F\n", temp_feels_like_celcius, temp_feels_like_fahr);
}

// Function to convert kelvin(temperature returned by the API) to celcius
pub fn kelvin_to_celcius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

// Function to convert celcius to fahrenheit
pub fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    (celcius * 9.0 / 5.0) + 32.0
}