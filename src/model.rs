use crate::forecast_weather_api::ForecastWeather;
use crate::ui::UiRequest;
use crate::FileStore;

use anyhow::*;
use esp_idf_svc::{
    hal::delay::FreeRtos,
    http::client::{Configuration as HttpConfig, EspHttpConnection},
    io::EspIOError,
    sntp::{EspSntp, SyncStatus},
    timer::EspTaskTimerService,
    wifi::{BlockingWifi, EspWifi},
};

use embedded_svc::{
    http::client::Client as HttpClient,
    utils::io,
    wifi::{AuthMethod, ClientConfiguration, Configuration},
};

use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;
use std::result::Result::Ok;
use std::str::Utf8Error;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::time::SystemTime;

use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;

use log::{info, warn};

#[derive(Debug)]
pub enum ModelRequest {
    UpdateCitiesInfo(Vec<CityInfo>),
    UpdateWifiCreds(String, String),
}

#[derive(Debug)]
enum HttpError {
    HttpGet(EspIOError),
    HttpSubmit(EspIOError),
    HttpRead(usize),
    Utf8Conversion(Utf8Error),
    HttpStatus(String),
}

#[derive(Debug, Clone)]
pub struct CityInfo {
    pub city_name: String,
    pub state: String,
    pub zipcode: String,
    pub timezone: String,
}

#[derive(Debug)]
pub struct CityForecast {
    pub temp: String,
    pub weather_descr: String,
    pub feels_like: String,
    pub uv: String,
    pub aqi: String,
    pub wind_speed: String,
    pub wind_gust: String,
    pub wind_dir: String,
    pub weekday_forecast_day_1: String,
    pub forecast_day_1: String,
    pub weekday_forecast_day_2: String,
    pub forecast_day_2: String,
    pub weekday_forecast_day_3: String,
    pub forecast_day_3: String,
    pub last_update: String,
}

#[derive(Debug)]
pub struct WifiCredentials {
    pub ssid: String,
    pub pass: String,
}

pub struct Model {
    wifi: BlockingWifi<EspWifi<'static>>,
    rx: Receiver<ModelRequest>,
    tx: Sender<UiRequest>,
    file_store: FileStore,
    wifi_credentials: WifiCredentials,
    cities_info: Vec<CityInfo>,
    weather_api_key: String,
}

impl Model {
    pub fn new(
        wifi: BlockingWifi<EspWifi<'static>>,
        rx: Receiver<ModelRequest>,
        tx: Sender<UiRequest>,
        file_store: FileStore,
    ) -> Self {
        let cities_info: Vec<CityInfo> = Vec::with_capacity(4);
        let wifi_credentials: WifiCredentials = WifiCredentials {
            ssid: String::new(),
            pass: String::new(),
        };
        Self {
            wifi,
            rx,
            tx,
            file_store,
            wifi_credentials,
            weather_api_key: String::new(),
            cities_info,
        }
    }

    pub fn run(&mut self) {
        // Read the text files from the SDCard
        self.read_wifi_txt_file();
        self.read_cities_txt_file();
        self.read_key_txt_file();

        // Update GUI
        self.send_wifi_credentials();
        self.send_cities_titles();
        self.send_cities_settings();

        // Connect to Wifi and establish connection with an SNTP server and send date time to GUI
        self.connect_to_wifi();
        let _sntp = self.create_sntp();
        self.send_date_time();

        // Update GUI cities forecast and then show home pane
        self.send_startup_pane_message("Waiting for cities forecasts".to_string());
        self.send_cities_forecasts();
        self.send_show_home_pane();

        // Create Two second timer thread
        //info!("---------- Creating Two Second Timer ----------");
        let two_second_timer_expired = Arc::new(AtomicBool::new(false));
        let two_second_timer_expired_clone = two_second_timer_expired.clone();
        let timer_service_02 = EspTaskTimerService::new().unwrap();
        let two_second_timer = timer_service_02
            .timer(move || {
                two_second_timer_expired_clone.store(true, Ordering::Relaxed);
            })
            .unwrap();

        // Let it trigger every 2 seconds
        two_second_timer.every(Duration::from_secs(2)).unwrap();

        // Create Ten minute thread
        //info!("---------- Creating Ten Minute Timer ----------");
        let ten_minute_timer_expired = Arc::new(AtomicBool::new(false));
        let ten_minute_timer_expired_clone = ten_minute_timer_expired.clone();
        let timer_service_03 = EspTaskTimerService::new().unwrap();
        let ten_minute_timer = timer_service_03
            .timer(move || {
                ten_minute_timer_expired_clone.store(true, Ordering::Relaxed);
            })
            .unwrap();

        // Let it trigger every 10 minutes
        ten_minute_timer
            .every(Duration::from_secs(10 * 60))
            .unwrap();

        loop {
            if let Ok(model_request) = self.rx.try_recv() {
                match model_request {
                    ModelRequest::UpdateWifiCreds(new_ssid, new_pass) => {
                        // Check to see if ssid or pass has changed
                        if self.wifi_credentials.ssid != new_ssid
                            || self.wifi_credentials.pass != new_pass
                        {
                            // Save locally
                            self.wifi_credentials.ssid = new_ssid;
                            self.wifi_credentials.pass = new_pass;

                            // Save to SDCard
                            self.write_wifi_txt_file();

                            // Restart WIFI with new ssid and pass
                            self.wifi.disconnect().unwrap();
                            self.connect_to_wifi();

                            self.send_startup_pane_message(
                                "Waiting for cities forecasts".to_string(),
                            );

                            // Update GUI time, cities forecast and then show home pane
                            self.send_date_time();
                            self.send_cities_forecasts();
                        }
                        self.send_show_home_pane();
                    }

                    ModelRequest::UpdateCitiesInfo(cities_info) => {
                        // Save Locally
                        self.cities_info = cities_info;

                        // Save to SDCard
                        self.write_cities_txt_file();

                        // Show statup pane
                        self.send_show_startup_pane();
                        self.send_startup_pane_message("Waiting for cities forecasts".to_string());

                        // Update GUI
                        self.send_cities_titles();
                        self.send_date_time();
                        self.send_cities_forecasts();
                        self.send_show_home_pane();
                    }
                }
            }

            if two_second_timer_expired.load(Ordering::Relaxed) {
                two_second_timer_expired.store(false, Ordering::Relaxed);
                self.send_date_time();
            }

            if ten_minute_timer_expired.load(Ordering::Relaxed) {
                ten_minute_timer_expired.store(false, Ordering::Relaxed);
                self.send_cities_forecasts();
            }

            FreeRtos::delay_ms(200);
        }
    }

    fn connect_to_wifi(&mut self) {
        self.send_show_startup_pane();
        self.send_startup_pane_message("Trying to connect to Wifi".to_string());

        // Sit in while loop forever until we can connect to the wifi network
        while let Err(_e) = self.connect_wifi() {
            warn!("---------- Error connecting to Wifi ---------");

            // Switch display from Startup pane to Wifi Settings pane so user can update wifi credentials
            self.send_show_wifi_settings_pane();
            self.send_wifi_settings_error_message(
                "Wifi connection failed check SSID and Password.".to_string(),
            );

            // Wait for the user to update wifi SSID or PASS
            loop {
                if let Ok(ModelRequest::UpdateWifiCreds(new_ssid, new_pass)) = self.rx.try_recv() {
                    self.wifi_credentials.ssid = new_ssid.clone();
                    self.wifi_credentials.pass = new_pass.clone();
                    self.write_wifi_txt_file();
                    self.send_show_startup_pane();
                    self.send_startup_pane_message("Trying to connect to Wifi".to_string());

                    break;
                } else {
                    FreeRtos::delay_ms(100);
                }
            }

            // Clear error message on Wifi Settings pane
            self.send_wifi_settings_error_message("".to_string());
        }
    }

    // Connect to wifi
    fn connect_wifi(&mut self) -> anyhow::Result<()> {
        let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
            ssid: self.wifi_credentials.ssid.as_str().try_into().unwrap(),
            bssid: None,
            auth_method: AuthMethod::WPA2Personal,
            password: self.wifi_credentials.pass.as_str().try_into().unwrap(),
            channel: None,
            ..Default::default()
        });

        self.wifi.set_configuration(&wifi_configuration)?;

        self.wifi.start()?;

        // If you are getting broken pipe on terminal after wifi.start then USB power is probably glitching,
        // you can try and reduce wifi TX power with the following unsafe method.  Your best option is to get
        // a better power source for the dev board; possibly a powered hub.
        //unsafe { esp_idf_svc::sys::esp_wifi_set_max_tx_power(34) };
        info!("Wifi started");
        self.wifi.connect()?;
        info!("Wifi connected");
        self.wifi.wait_netif_up()?;
        info!("Wifi netif up");

        Ok(())
    }

    fn create_sntp(&self) -> anyhow::Result<Box<EspSntp<'static>>> {
        self.send_startup_pane_message(
            "Waiting for Network Time Protocol server connection".to_string(),
        );

        let sntp = EspSntp::new_default()?;
        while sntp.get_sync_status() != SyncStatus::Completed {
            FreeRtos::delay_ms(300);
        }
        info!("Time Sync Completed");

        Ok(Box::new(sntp))
    }

    fn read_wifi_txt_file(&mut self) {
        let lines = self.file_store.read_lines_from_file("wifi.txt").unwrap();

        self.wifi_credentials = WifiCredentials {
            ssid: lines[0].clone(),
            pass: lines[1].clone(),
        };
    }

    fn write_wifi_txt_file(&mut self) {
        let lines = format!(
            "{}\n{}",
            self.wifi_credentials.ssid.clone(),
            self.wifi_credentials.pass.clone()
        );

        self.file_store
            .write_lines_to_file("wifi.txt", lines.as_bytes())
            .unwrap();
    }

    fn read_cities_txt_file(&mut self) {
        let lines = self.file_store.read_lines_from_file("cities.txt").unwrap();

        for line in lines {
            let city: Vec<&str> = line.split(',').collect();

            self.cities_info.push(CityInfo {
                city_name: city[0].to_string(),
                state: city[1].to_string(),
                zipcode: city[2].to_string(),
                timezone: city[3].to_string(),
            });
        }
    }

    fn write_cities_txt_file(&mut self) {
        // Create lines that will be written to SDCard
        let mut lines = String::new();
        for city in 0..self.cities_info.len() {
            let line = format!(
                "{},{},{},{}\n",
                self.cities_info[city].city_name,
                self.cities_info[city].state,
                self.cities_info[city].zipcode,
                self.cities_info[city].timezone
            );

            lines += line.as_str();
        }

        // Save to SDCard
        self.file_store
            .write_lines_to_file("cities.txt", lines.as_bytes())
            .unwrap();
    }

    fn read_key_txt_file(&mut self) {
        let lines = self.file_store.read_lines_from_file("key.txt").unwrap();
        self.weather_api_key = lines[0].clone();
    }

    fn send_startup_pane_message(&self, message: String) {
        self.tx.send(UiRequest::SetStartupMessage(message)).unwrap();
    }

    fn send_show_startup_pane(&self) {
        self.tx.send(UiRequest::ShowStartup).unwrap();
    }

    fn send_show_home_pane(&self) {
        self.tx.send(UiRequest::ShowHome).unwrap();
    }

    fn send_show_wifi_settings_pane(&self) {
        self.tx.send(UiRequest::ShowWifiSettings).unwrap();
    }

    fn send_wifi_settings_error_message(&self, message: String) {
        self.tx
            .send(UiRequest::SetWifiSettingsErrorMessage(message))
            .unwrap();
    }

    fn send_wifi_credentials(&self) {
        self.tx
            .send(UiRequest::SetWifiCreds(
                self.wifi_credentials.ssid.clone(),
                self.wifi_credentials.pass.clone(),
            ))
            .unwrap();
    }

    fn send_date_time(&mut self) {
        let st_now = SystemTime::now();
        let dt: DateTime<Utc> = st_now.into();

        for city in 0..self.cities_info.len() {
            let timezone: String = self.cities_info[city].timezone.clone();
            let city_timezone: Tz = timezone.parse().unwrap();
            let city_datetime = dt.with_timezone(&city_timezone);
            let city_time = format!("{}", city_datetime.format("%I:%M%P"));
            let city_date = format!("{}", city_datetime.format("%a %b %d %Y"));

            self.tx
                .send(UiRequest::SetCityTime(
                    city,
                    city_time.to_string(),
                    city_date.to_string(),
                ))
                .unwrap();
        }
    }

    fn send_cities_titles(&mut self) {
        self.tx
            .send(UiRequest::SetCitiesTitles(self.cities_info.clone()))
            .unwrap();
    }

    fn send_cities_settings(&mut self) {
        self.tx
            .send(UiRequest::SetCitiesSettings(self.cities_info.clone()))
            .unwrap();
    }

    fn send_cities_forecasts(&mut self) {
        for city in 0..self.cities_info.len() {
            if let Some(cf) = self.fetch_city_forecast(city) {
                let fw: ForecastWeather = serde_json::from_str(&cf).unwrap();
                let temp = format!("{:.0}F", fw.current.temp_f);
                let weather_descr = fw.current.condition.text;
                let feels_like = format!("{:.0}F", fw.current.feelslike_f);
                let uv = format!("{:.0}", fw.current.uv);
                let aqi = format!("{}", fw.current.air_quality.us_epa_index);
                let wind_speed = format!("{:.0}", fw.current.wind_mph);
                let wind_gust = format!("{:.0}", fw.current.gust_mph);
                let wind_dir = fw.current.wind_dir;

                let mut date_time = Utc
                    .timestamp_opt(fw.forecast.forecastday[0].date_epoch, 0)
                    .unwrap();
                let weekday_forecast_day_1 = format!("{}", date_time.format("%a %d"));
                let mut day_hi_temp = fw.forecast.forecastday[0].day.maxtemp_f;
                let mut day_lo_temp = fw.forecast.forecastday[0].day.mintemp_f;
                let forecast_day_1 = format!("{:.0}F\n{:.0}F", day_hi_temp, day_lo_temp);

                date_time = Utc
                    .timestamp_opt(fw.forecast.forecastday[1].date_epoch, 0)
                    .unwrap();
                let weekday_forecast_day_2 = format!("{}", date_time.format("%a %d"));
                day_hi_temp = fw.forecast.forecastday[1].day.maxtemp_f;
                day_lo_temp = fw.forecast.forecastday[1].day.mintemp_f;
                let forecast_day_2 = format!("{:.0}F\n{:.0}F", day_hi_temp, day_lo_temp);

                date_time = Utc
                    .timestamp_opt(fw.forecast.forecastday[2].date_epoch, 0)
                    .unwrap();
                let weekday_forecast_day_3 = format!("{}", date_time.format("%a %d"));
                day_hi_temp = fw.forecast.forecastday[2].day.maxtemp_f;
                day_lo_temp = fw.forecast.forecastday[2].day.mintemp_f;
                let forecast_day_3 = format!("{:.0}F\n{:.0}F", day_hi_temp, day_lo_temp);

                let dt_last = Utc.timestamp_opt(fw.current.last_updated_epoch, 0).unwrap();
                let city_tz: Tz = self.cities_info[city].timezone.clone().parse().unwrap();
                let dt_last_with_tz = dt_last.with_timezone(&city_tz);
                let dt_last_str = dt_last_with_tz.format("%D %I:%M%P");
                let last_update = format!("Last update: {}", dt_last_str);

                self.tx
                    .send(UiRequest::SetCityForecast(
                        city,
                        CityForecast {
                            temp,
                            weather_descr,
                            feels_like,
                            uv,
                            aqi,
                            wind_speed,
                            wind_gust,
                            wind_dir,
                            weekday_forecast_day_1,
                            forecast_day_1,
                            weekday_forecast_day_2,
                            forecast_day_2,
                            weekday_forecast_day_3,
                            forecast_day_3,
                            last_update,
                        },
                    ))
                    .unwrap();
            }
        }
    }

    fn fetch_city_forecast(&mut self, city_id: usize) -> Option<String> {
        let url = format!(
            "http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=3&aqi=yes&alerts=no",
            self.weather_api_key,
            self.cities_info[city_id].zipcode.clone()
        );

        let httpconnection = EspHttpConnection::new(&HttpConfig {
            use_global_ca_store: true,
            crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
            timeout: Some(Duration::from_secs(10)),
            ..Default::default()
        })
        .unwrap();

        let mut httpclient = HttpClient::wrap(httpconnection);

        match request(&mut httpclient, url.as_str()) {
            Ok(json_str) => Some(json_str),

            Err(e) => match e {
                HttpError::HttpGet(e) => {
                    warn!("http get error = {}", e.0);
                    None
                }

                HttpError::HttpSubmit(e) => {
                    warn!("http submit error = {:?}", e.0);
                    None
                }

                HttpError::HttpRead(e) => {
                    warn!("http read error, bytes read = {}", e);
                    None
                }

                HttpError::Utf8Conversion(e) => {
                    warn!("http json error = {}", e);
                    None
                }

                HttpError::HttpStatus(e) => {
                    warn!("http status code error = {}", e);
                    None
                }
            },
        }
    }
}

// HTTP request
fn request(
    http_client: &mut HttpClient<EspHttpConnection>,
    url: &str,
) -> Result<String, HttpError> {
    let mut response = http_client
        .get(url)
        .map_err(HttpError::HttpGet)?
        .submit()
        .map_err(HttpError::HttpSubmit)?;

    match response.status() {
        200 => {
            let mut data: Vec<u8> = vec![0; 60000];
            let bytes_read = io::try_read_full(&mut response, data.as_mut_slice())
                .map_err(|e| HttpError::HttpRead(e.1))?;
            //println!("bytes read = {}", bytes_read);

            let json_str = std::str::from_utf8(&data.as_mut_slice()[0..bytes_read])
                .map_err(HttpError::Utf8Conversion)?
                .to_string();

            Ok(json_str)
        }

        _ => {
            let err_msg = format!("Bad HTTP status - {}", response.status());
            Err(HttpError::HttpStatus(err_msg))
        }
    }
}
