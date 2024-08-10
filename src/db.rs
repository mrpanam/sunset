
use serde::Deserialize;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use crate::model::{City,Weather,WeatherDetails,Sun };
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}



use crate::weather:: WeatherResponse;

pub async fn save_weather_data(data: &WeatherResponse) -> Result<(), Box<dyn Error>> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000/").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    }).await?;
    db.use_ns("test").use_db("meteo").await?;

    let mut weather_data = serde_json::to_value(data)?;
    if let Some(obj) = weather_data.as_object_mut() {
        obj.remove("id");
        
         // Remove the id field
    }

      // Check if the city already exists
      let existing_city: Vec<Record> = db
      .query("SELECT * FROM City WHERE name = $name")
      .bind(("name", &data.name))
      .await?
      .take(0)?;

  let city_id = if existing_city.is_empty() {
      // City does not exist, create a new one
      let city: Vec<Record> = db
          .create("City")
          .content(City {
              name: data.name.clone(),
              country: data.sys.country.clone(),
              timezone: data.timezone,
              long: data.coord.lon,
              lat: data.coord.lat,              
              date: data.dt,
          }).await?;
      city[0].id.clone()
  } else {
      // City exists, use the existing city ID
      existing_city[0].id.clone()
  };
    let weather:Vec<Record> =db
        .create("Weather")
        .content(
            Weather{
                main: data.weather[0].main.clone(),     
                description: data.weather[0].description.clone(),
                temperature: data.main.temp,
                feels_like: data.main.feels_like,
                temperature_min: data.main.temp_min,
                temperature_max: data.main.temp_max,
                humidity: data.main.humidity,
                pressure: data.main.pressure,
                date : data.dt,
                visibility: data.visibility,
                city_id: city_id.clone(),         
        })
        .await?;

    let weatherdetails:Vec<Record> =db
        .create("WeatherDetails")
        .content(
            WeatherDetails{
                wind_speed: data.wind.speed,
                wind_deg: data.wind.deg,
                wind_gust: Some(data.wind.gust.unwrap_or(0.0)),
                clouds: data.clouds.all,
                rain: data.rain.as_ref().map_or(0.0, |rain| rain.rainfall.get("1h").cloned().unwrap_or(0.0)),
                date : data.dt,
                city_id: city_id.clone(),         
        })
            
        .await?;

    let sunset:Vec<Record> =db.create("Sunset")
    .content(Sun{
        city_id: city_id.clone(),
        sunrise: data.sys.sunrise,
        sunset: data.sys.sunset,
        date : data.dt
    })         
       .await?;

    let sunset2:Vec<Sun> =db.create("Sunset2")
    .content(Sun{
        city_id: city_id.clone(),
        sunrise: data.sys.sunrise,
        sunset: data.sys.sunset,
        date : data.dt
    })         
       .await?;

           
    println!("Data saved successfully for {}.", data.name);
    Ok(())
}