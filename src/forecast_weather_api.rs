use serde::{Deserialize, Serialize};

/// Weather condition
#[derive(Deserialize, Serialize, Debug)]
pub struct Condition {
    /// Weather condition text
    pub text: String,
    /// Weather condition icon
    pub icon: String,
    /// Weather condition unique code
    pub code: i32,
}

/// Location
#[derive(Deserialize, Serialize, Debug)]
pub struct Location {
    /// Location name
    pub name: String,
    /// Region or state of the location, if available
    pub region: String,
    /// Location country
    pub country: String,
    /// geo location, latitude
    pub lat: f64,
    /// geo location, longitude
    pub lon: f64,
    /// Timezone ID
    pub tz_id: String,
    /// Local date and time in unix time
    pub localtime_epoch: i64,
    /// Local date and time
    pub localtime: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AirQuality {
    /// Carbon Monoxide (ug/m3)
    pub co: f64,
    /// Ozone (ug/m3)
    pub no2: f64,
    /// Nitogen dioxide (ug/m3)
    pub o3: f64,
    /// Sulphur dioxide (ug/m3)
    pub so2: f64,
    /// PM2.5 (ug/m3)
    pub pm2_5: f64,
    /// PM10 (ug/m3)
    pub pm10: f64,
    /// US - EPA standard: 1 means Good, 2 means Moderate,
    /// 3 means Unhealthy for sensitive group, 4 means Unhealthy,
    /// 5 means very unhealthy, 6 means hazardous
    #[serde(rename = "us-epa-index")]
    pub us_epa_index: u16,
    /// Uk Defra Index
    #[serde(rename = "gb-defra-index")]
    pub gb_defra_index: u16,
}

/// Current
#[derive(Deserialize, Serialize, Debug)]
pub struct Current {
    /// Local time when the real time data was updated in unix time.
    pub last_updated_epoch: i64,
    /// Local time when the real time data was updated.
    pub last_updated: String,
    /// Temperature in celsius
    pub temp_c: f64,
    /// Temperature in fahrenheit
    pub temp_f: f64,
    /// Whether to show day condition icon or night icon (1=yes,0=no)
    pub is_day: i32,
    /// Weather condition
    pub condition: Condition,
    /// Wind speed in miles per hour
    pub wind_mph: f64,
    /// Wind speed in kilometer per hour
    pub wind_kph: f64,
    /// Wind direction in degrees
    pub wind_degree: i32,
    /// Wind direction as 16 point compass. e.g.: NSW
    pub wind_dir: String,
    /// Pressure in millibars
    pub pressure_mb: f64,
    /// Pressure in inches
    pub pressure_in: f64,
    /// Humidity as percentage
    pub humidity: i32,
    /// Cloud cover as percentage
    pub cloud: i32,
    /// Feels like temperature in celsius
    pub feelslike_c: f64,
    /// Feels like temperature in fahrenheit
    pub feelslike_f: f64,
    /// Visibility in kilometer
    pub vis_km: f64,
    /// Visibility in miles
    pub vis_miles: f64,
    /// UV Index
    pub uv: f64,
    /// Wind gust in miles per hour
    pub gust_mph: f64,
    /// Wind gust in kilometer per hour
    pub gust_kph: f64,
    /// Air quality data
    pub air_quality: AirQuality,
}

#[derive(Deserialize, Serialize, Debug)]
/// forecats weather report in a nested struct
pub struct ForecastWeather {
    /// Location
    pub location: Location,
    /// Current weather at this location
    pub current: Current,
    /// Weather forcast
    pub forecast: Forecast,
}

#[derive(Deserialize, Serialize, Debug)]
/// forecastweather report in a nested struct
pub struct Forecast {
    /// forecast days
    pub forecastday: Vec<ForecastDay>,
}

#[derive(Deserialize, Serialize, Debug)]
/// forecastday
pub struct ForecastDay {
    /// Forecast date
    pub date: String,
    /// Forecast date as unix time.
    pub date_epoch: i64,
    /// Day item
    pub day: Day,
    /// Astro item
    pub astro: Astro,
    /// Hour items
    pub hour: Vec<Hour>,
}

#[derive(Deserialize, Serialize, Debug)]
/// Day
pub struct Day {
    /// Maximum temperature in celsius for the day.
    pub maxtemp_c: f64,
    /// Maximum temperature in fahrenheit for the day
    pub maxtemp_f: f64,
    /// Minimum temperature in celsius for the day
    pub mintemp_c: f64,
    /// Minimum temperature in fahrenheit for the day
    pub mintemp_f: f64,
    /// Average temperature in celsius for the day
    pub avgtemp_c: f64,
    /// Average temperature in fahrenheit for the day
    pub avgtemp_f: f64,
    /// Maximum wind speed in miles per hour
    pub maxwind_mph: f64,
    /// Maximum wind speed in kilometer per hour
    pub maxwind_kph: f64,
    /// Total precipitation in milimeter
    pub totalprecip_mm: f64,
    /// Total precipitation in inches
    pub totalprecip_in: f64,
    /// Total snowfall in centimeters
    pub totalsnow_cm: f64,
    /// Average visibility in kilometer
    pub avgvis_km: f64,
    /// Average visibility in miles
    pub avgvis_miles: f64,
    /// Average humidity as percentage
    pub avghumidity: i32,
    /// Will it will rain or not 1 = Yes 0 = No
    pub daily_will_it_rain: i32,
    /// Chance of rain as percentage
    pub daily_chance_of_rain: i32,
    /// Will it will snow or not 1 = Yes 0 = No
    pub daily_will_it_snow: i32,
    /// Chance of snow as percentage
    pub daily_chance_of_snow: i32,
    /// Weather conditions
    pub condition: Condition,
    /// UV index
    pub uv: f64,
}

#[derive(Deserialize, Serialize, Debug)]
/// astronomical
pub struct Astro {
    /// Sunrise time
    pub sunrise: String,
    /// Sunset time
    pub sunset: String,
    /// Moonrise time
    pub moonrise: String,
    /// Moonset time
    pub moonset: String,
    /// Moon phases
    pub moon_phase: String,
    /// Moon illumination as %
    pub moon_illumination: f64,
    /// Is moon up
    pub is_moon_up: i32,
    /// Is sun up
    pub is_sun_up: i32,
}

#[derive(Deserialize, Serialize, Debug)]
/// Hour
pub struct Hour {
    /// Time as epoch
    pub time_epoch: i64,
    /// Date and time
    pub time: String,
    /// Temperature in celsius
    pub temp_c: f64,
    /// Temperature in fahrenheit
    pub temp_f: f64,
    /// Whether to show day condition icon or night icon 1 = Yes 0 = No
    pub is_day: i32,
    /// Weather condition
    pub condition: Condition,
    /// Wind speed in miles per hour
    pub wind_mph: f64,
    /// Wind speed in kilometer per hour
    pub wind_kph: f64,
    /// Wind direction in degrees
    pub wind_degree: i32,
    /// Wind direction as 16 point compass. e.g.: NSW
    pub wind_dir: String,
    /// Pressure in millibars
    pub pressure_mb: f64,
    /// Pressure in inches
    pub pressure_in: f64,
    /// Precipitation amount in millimeters
    pub precip_mm: f64,
    /// Precipitation amount in inches
    pub precip_in: f64,
    /// Snowfall in centimeters
    pub snow_cm: f64,
    /// Humidity as percentage
    pub humidity: i32,
    /// Cloud cover as percentage
    pub cloud: i32,
    /// Feels like temperature in celsius
    pub feelslike_c: f64,
    /// Feels like temperature in fahrenheit
    pub feelslike_f: f64,
    /// Windchill temperature in celsius
    pub windchill_c: f64,
    /// Windchill temperature in fahrenheit
    pub windchill_f: f64,
    /// Heat Index temperature in celsius
    pub heatindex_c: f64,
    /// Heat Index temperature in fahrenheit
    pub heatindex_f: f64,
    /// Dewpoint temperature in celsius
    pub dewpoint_c: f64,
    /// Dewpoint temperature in fahrenheit
    pub dewpoint_f: f64,
    /// Will it will rain or not 1 = Yes 0 = No
    pub will_it_rain: i32,
    /// Chance of rain as percentage
    pub chance_of_rain: i32,
    /// Will it snow or not 1 = Yes 0 = No
    pub will_it_snow: i32,
    /// Chance of snow as percentage
    pub chance_of_snow: i32,
    /// Visibility in kilometer
    pub vis_km: f64,
    /// Visibility in miles
    pub vis_miles: f64,
    /// Wind gust in miles per hour
    pub gust_mph: f64,
    /// Wind gust in kilometer per hour
    pub gust_kph: f64,
    /// UV Index
    pub uv: f64,
    /*
    /// Shortwave solar radiation or Global horizontal irradiation (GHI) W/m²
    pub short_rad: f64,
    /// Diffuse Horizontal Irradiation (DHI) W/m²
    pub diff_rad: f64,
    */
}
