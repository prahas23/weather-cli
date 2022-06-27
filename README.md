# weather-cli

weather-cli is a command-line interface app written in the Rust programming language. The app uses OpenWeatherMap's API to fetch the current weather of any city in the world.

## Building the source code locally
**By cloning this repository:**
```
git clone https://github.com/prahas23/weather-cli.git
cd weather-cli
cargo build
```

## Running the program
Name of the city and the two letter country code should be passed as arguments while running the app.
```
cargo run <city_name> <country_code>
```
**Examples:**
```
cargo run London GB
cargo run Stockholm SE
cargo run Amsterdam NL
```
**Output:**

![image](https://user-images.githubusercontent.com/59818767/176035732-b4a1df80-f10b-4463-8406-69e1197ac957.png)

![image](https://user-images.githubusercontent.com/59818767/176036519-2a8d9ab1-9c10-4836-a2c9-84bb711660a1.png)
