use core::result;
use std::{collections::HashMap, io};

type MyResult<T> = result::Result<T, io::Error>;

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
    let w0 = &weather_res;
    assert_eq!(w0.is_ok(), true);
    assert_eq!(w0.is_err(), false);

    // Some(WeatherReport { condition: "sunny" })
    println!("weather_res ok: {:?}", w0.as_ref().ok());

    handle_weather_res(&weather_res);

    let loc1 = "B";
    let weather_res1: Result<WeatherReport, io::Error> = WeatherReport::new(loc1);
    let w1 = &weather_res1;
    assert_eq!(w1.is_ok(), false);
    assert_eq!(w1.is_err(), true);

    let res = error_result(true);
    // res: None
    println!("res: {:?}", res.as_ref().ok());
    println!("res default: {:?}", res.unwrap_or(0));

    // weather_res1 error: Some(Custom { kind: Other, error: "no weather found" })
    println!("weather_res1 error: {:?}", w1.as_ref().err());
    handle_weather_res(&weather_res1);

    let my_res = ret_myresult();
    println!("my result: {:?}", my_res);
}

fn handle_weather_res(weather_res: &Result<WeatherReport, io::Error>) {
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

fn error_result(ret_err: bool) -> Result<i32, String> {
    if ret_err {
        return Err("error result".to_string());
    }

    return Ok(1);
}

fn ret_myresult() -> MyResult<i32> {
    Ok(10)
}
