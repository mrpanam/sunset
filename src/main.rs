mod weather;
mod helper;
mod db;
mod model;


mod request;


#[tokio::main] 
async fn main() {
    let weather_result = request::get_weather_for_locations().await;

    println!("il est : {}", chrono::offset::Local::now());
    println!("pour Titi et Nénette");
    

    if let Ok(result) = weather_result {
        for weather_data in result {
            if let Err(e) = db::save_weather_data(&weather_data).await {
                println!("Failed to save data to database: {}", e);
            }
        }
    } else {
        // Handle the error case here
        println!("Failed to get weather data");
    }

    /* 
    let today = chrono::offset::Local::now();

    println!("Coucou nous sommes le {}", today.format("%d/%m/%Y"));
    println!(" ");

    let ville = city::City {
        name: String::from("Bruyeres"), //"Paris",
        lat: 49.15745966301978,
        long: 2.3387259142297294,
    };
    println!("{}", ville);

    let (sunrise, sunset) =
        City::get_sunrise_sunset(&ville, today.year(), today.month(), today.day());
    City::sun_time(sunrise, sunset);

    let _ = io::stdin().read_line(&mut String::new());*/
}
