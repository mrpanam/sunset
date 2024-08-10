use reqwest;
use crate::weather;
use serde_json;




pub async fn get_weather_for_locations() -> Result<Vec<weather::WeatherResponse>, Box<dyn std::error::Error>> {
    let locations = vec![
        weather::Coord { lon: 2.338725, lat: 49.157459 },
        weather::Coord { lon: -0.075166, lat: 49.300865 },
        weather::Coord { lon: 1.363993, lat: 44.860146 },
        weather::Coord { lon: 5.729384, lat: 34.851248},
        weather::Coord { lon: -1.230814, lat: 44.203632 },
        weather::Coord { lon: 1.444980, lat: 43.604232 }

        

    ];
    let mut all_weather_data = Vec::new();
    


    for location in &locations {
        
        let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units=metric&appid=3e706d704abbc847d0b5ebe9c6969d0d", location.lat, location.lon);
        let client = reqwest::Client::new();
        
        let response = client.get(&url).send().await?;
               if response.status().is_success() {
            let body = response.text().await?;
            let weather = serde_json::from_str::<weather::WeatherResponse>(&body)?;
            
            all_weather_data.push(weather);
        } else {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get weather")));
        }
    }

return Ok(all_weather_data);
     
   
}


//