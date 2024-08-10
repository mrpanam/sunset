use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize,Serialize,Debug)]

pub struct WeatherResponse {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: u32,
    pub wind: Wind,
    pub rain: Option<Rain>,
    pub clouds: Clouds,
    pub dt: u64,
    pub sys: Sys,
    pub timezone: i32,    
    pub id: u32,
    pub name: String,
    pub cod: u32,
}

#[derive(Deserialize,Serialize)]
#[derive(Debug)]
pub struct Coord {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Deserialize,Serialize)]
#[derive(Debug)]
pub struct Weather {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize,Serialize)]
#[derive(Debug)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: u32,
    pub humidity: u32,
    pub sea_level: u32,
    pub grnd_level: u32,
}

#[derive(Deserialize,Serialize)]
#[derive(Debug)]
pub struct Wind {
    pub speed: f32,
    pub deg: f32,
    pub gust: Option<f32>,
}

#[derive(Deserialize,Serialize)]
#[derive(Debug)]
pub struct Rain {
    #[serde(flatten)] 
    pub rainfall: HashMap<String, f32>,
}

#[derive(Deserialize,Serialize)]
#[derive(Debug)]
pub struct Clouds {
    pub all: u32,
}

#[derive(Deserialize,Serialize)]
#[derive(Debug)]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_: Option<u32>,
    pub id: Option<u32>,
    pub country: String,
    pub sunrise: u64,
    pub sunset: u64,
}