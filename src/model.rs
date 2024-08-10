use serde:: {Serialize,Deserialize};
use surrealdb::sql::Thing;

#[derive(Serialize,Debug)]
pub struct City {    
    pub name: String,
    pub lat: f32,
    pub long: f32,
    pub country: String,    
    pub timezone: i32,   
    pub date: u64
}

 
#[derive(Serialize,Deserialize,Debug)]
pub struct Sun {
    pub city_id: Thing,
    pub sunrise: u64,
    pub sunset: u64,
    pub date: u64
}


#[derive(Serialize,Debug)]
pub struct Weather {
    pub main: String,
    pub description: String,
    pub temperature: f32,
    pub feels_like: f32,
    pub temperature_min: f32,
    pub temperature_max: f32,
    pub humidity: u32,
    pub pressure: u32,
    pub date : u64,
    pub visibility: u32,
    pub city_id: Thing

}
#[derive(Serialize,Debug)]
pub struct WeatherDetails {
    pub wind_speed: f32,
    pub wind_deg:f32,
    pub wind_gust: Option<f32>,
    pub clouds: u32,
    pub rain: f32,
    pub date : u64,
    pub city_id: Thing
}