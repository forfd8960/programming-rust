use std::{collections::HashMap, io};

#[derive(Debug)]
struct WeatherReport {
    condition: String,
}

impl WeatherReport {
    fn new(location: &str) -> Result<Self, io::Error> {
        let mut data: HashMap<String, String> = HashMap::new();
        data.insert("A".to_string(), "sunny".to_string());
        data.insert("b".to_string(), "rain".to_string());

        let weather = data.get(location);
        if weather.is_some() {
            return Ok(Self {
                condition: weather.unwrap().to_string(),
            });
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "no weather found".to_string(),
        ))
    }
}

fn main() {
    let loc = "A";
    let weather_res = WeatherReport::new(loc);
    handle_weather_res(weather_res);

    let loc1 = "B";
    let weather_res1: Result<WeatherReport, io::Error> = WeatherReport::new(loc1);
    handle_weather_res(weather_res1);
}

fn handle_weather_res(weather_res: Result<WeatherReport, io::Error>) {
    match weather_res {
        Ok(w) => {
            println!("weather is: {:?}", w);
            println!("weather is: {:?}", w.condition);
        }
        Err(e) => {
            println!("get weather error: {}", e);
        }
    }
}
