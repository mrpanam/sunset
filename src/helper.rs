use chrono::{DateTime, Local, TimeZone};


pub fn unix_timestamp_to_datetime(seconds: i64) -> DateTime<Local> {
    let datetime = Local.timestamp_opt(seconds, 0).unwrap();
    datetime
}



/*pub fn get_sunrise_sunset(
    &self,
    year: i32,
    month: u32,
    day: u32,
) -> (DateTime<Local>, DateTime<Local>) {
    let (sunrise_seconds, sunset_seconds) = sunrise_sunset(self.lat, self.long, year, month, day);

    let sunrise: DateTime<Local> = City::seconds_to_datetime(sunrise_seconds);
    let sunset: DateTime<Local> = City::seconds_to_datetime(sunset_seconds);
    println!(" ");
    println!(
        "le soleil se lève à: {}, le soleil se couche à: {}",
        sunrise.format("%H:%M:%S"),
        sunset.format("%H:%M:%S")
    );
    (sunrise, sunset)
}

// Calculate and print sunlight duration in hours and minutes
pub fn sun_time(sunrise_time: DateTime<Local>, sunset_time: DateTime<Local>) {
    let duration = sunset_time.signed_duration_since(sunrise_time);
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    println!("");
    println!(
        "Durée d'ensoleillement pour la journée: {} hours {} minutes",
        hours, minutes
    );*/

