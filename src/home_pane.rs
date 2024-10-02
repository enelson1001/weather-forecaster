//============================================================================
//                              Home Pane
//============================================================================
use crate::lvgl_misc;
use crate::model::{CityForecast, CityInfo};
use cstr_core::CString;
use lvgl::font::Font;
use lvgl::misc::area::pct;
use lvgl::style::{FlexAlign, FlexFlow, Layout, Opacity, Style};
use lvgl::widgets::Label;
use lvgl::{Align, Color, Obj, Part, Screen, TextAlign, Widget};
use lvgl_sys::*;

pub struct CityLabels<'a> {
    pub title: Label<'a>,
    pub time: Label<'a>,
    pub date: Label<'a>,
    pub temp: Label<'a>,
    pub weather_descr: Label<'a>,
    pub uv: Label<'a>,
    pub feels_like: Label<'a>,
    pub aqi: Label<'a>,
    pub wind_speed: Label<'a>,
    pub wind_gust: Label<'a>,
    pub wind_dir: Label<'a>,
    pub title_forecast_day_1: Label<'a>,
    pub forecast_day_1: Label<'a>,
    pub title_forecast_day_2: Label<'a>,
    pub forecast_day_2: Label<'a>,
    pub title_forecast_day_3: Label<'a>,
    pub forecast_day_3: Label<'a>,
    pub last_update: Label<'a>,
}

pub struct HomePane<'a> {
    home_pane: Obj<'a>,
    cities_labels: Vec<CityLabels<'a>>,
}

impl<'a> HomePane<'a> {
    pub fn new(home_pane_parent: &'a mut Screen) -> Self {
        let mut home_pane = Obj::create(home_pane_parent).unwrap();
        home_pane.set_size(800, 429);
        home_pane.set_align(Align::TopLeft, 0, 51);
        home_pane.add_style(Part::Main, Box::leak(home_pane_style()));

        let mut cities_labels: Vec<CityLabels> = Vec::new();

        // Create 4 city panes
        for _i in 1..5 {
            // city pane
            let mut city_pane = Obj::create(&mut home_pane).unwrap();
            city_pane.set_size(183, pct(100));
            city_pane.add_style(Part::Main, Box::leak(city_pane_style()));

            // City title
            let mut title = Label::create(&mut city_pane).unwrap();
            title.set_width(163);
            title.set_align(Align::TopMid, 0, 5);
            let mut text = CString::new("").unwrap();
            title.set_text(text.as_c_str()).unwrap();
            title.add_style(Part::Main, Box::leak(city_title_style()));

            // city local time title
            let mut label = Label::create(&mut city_pane).unwrap();
            label.set_align(Align::TopMid, 0, 53);
            text = CString::new("Local Time").unwrap();
            label.set_text(text.as_c_str()).unwrap();

            // city local time
            let mut time = Label::create(&mut city_pane).unwrap();
            time.set_align(Align::TopMid, 0, 66);
            text = CString::new("").unwrap();
            time.set_text(text.as_c_str()).unwrap();
            time.add_style(Part::Main, Box::leak(time_style()));

            // city local date
            let mut date = Label::create(&mut city_pane).unwrap();
            date.set_align(Align::TopMid, 0, 93);
            text = CString::new("").unwrap();
            date.set_text(text.as_c_str()).unwrap();

            // city - local weather title label
            label = Label::create(&mut city_pane).unwrap();
            label.set_align(Align::TopMid, 0, 125);
            text = CString::new("Local Weather").unwrap();
            label.set_text(text.as_c_str()).unwrap();

            // city - local temperature
            let mut temp = Label::create(&mut city_pane).unwrap();
            temp.set_align(Align::TopMid, 0, 136);
            text = CString::new("").unwrap();
            temp.set_text(text.as_c_str()).unwrap();
            temp.add_style(Part::Main, Box::leak(temperature_style()));

            // city - weather description label, wrap text for long descriptions
            let mut weather_descr = Label::create(&mut city_pane).unwrap();
            weather_descr.set_width(150);
            weather_descr.set_align(Align::TopMid, 0, 170);
            let text = CString::new("").unwrap();
            weather_descr.set_text(text.as_c_str()).unwrap();
            weather_descr.add_style(Part::Main, Box::leak(font_12_red_style()));

            // city - uv title label
            label = Label::create(&mut city_pane).unwrap();
            label.set_align(Align::Center, -55, 25);
            let text = CString::new("UV").unwrap();
            label.set_text(text.as_c_str()).unwrap();

            // city - uv
            let mut uv = Label::create(&mut city_pane).unwrap();
            uv.set_align(Align::Center, -55, 43);
            uv.set_width(40);
            let text = CString::new("").unwrap();
            uv.set_text(text.as_c_str()).unwrap();
            uv.add_style(Part::Main, Box::leak(color_scale_style()));

            // city - feels like title label
            label = Label::create(&mut city_pane).unwrap();
            label.set_align(Align::Center, 0, 25);
            let text = CString::new("Feels Like").unwrap();
            label.set_text(text.as_c_str()).unwrap();

            // city - feels like temperature
            let mut feels_like = Label::create(&mut city_pane).unwrap();
            feels_like.set_align(Align::Center, 0, 43);
            let text = CString::new("").unwrap();
            feels_like.set_text(text.as_c_str()).unwrap();
            feels_like.add_style(Part::Main, Box::leak(font_14_blu_style()));

            // city - air quality index title label
            label = Label::create(&mut city_pane).unwrap();
            label.set_align(Align::Center, 55, 25);
            let text = CString::new("AQI").unwrap();
            label.set_text(text.as_c_str()).unwrap();

            // city - air quality index
            let mut aqi = Label::create(&mut city_pane).unwrap();
            aqi.set_align(Align::Center, 55, 43);
            aqi.set_width(40);
            let text = CString::new("").unwrap();
            aqi.set_text(text.as_c_str()).unwrap();
            aqi.add_style(Part::Main, Box::leak(color_scale_style()));

            // city - wind speed title label
            label = Label::create(&mut city_pane).unwrap();
            label.set_align(Align::Center, -55, 65);
            let text = CString::new("Wind").unwrap();
            label.set_text(text.as_c_str()).unwrap();

            // city - wind speed
            let mut wind_speed = Label::create(&mut city_pane).unwrap();
            wind_speed.set_align(Align::Center, -55, 80);
            let text = CString::new("").unwrap();
            wind_speed.set_text(text.as_c_str()).unwrap();
            wind_speed.add_style(Part::Main, Box::leak(font_14_blu_style()));

            // city - wind gust title label
            label = Label::create(&mut city_pane).unwrap();
            label.set_align(Align::Center, 0, 65);
            let text = CString::new("Gust").unwrap();
            label.set_text(text.as_c_str()).unwrap();

            // city - wind gust
            let mut wind_gust = Label::create(&mut city_pane).unwrap();
            wind_gust.set_align(Align::Center, 0, 80);
            let text = CString::new("").unwrap();
            wind_gust.set_text(text.as_c_str()).unwrap();
            wind_gust.add_style(Part::Main, Box::leak(font_14_blu_style()));

            // city - wind direction title label
            label = Label::create(&mut city_pane).unwrap();
            label.set_align(Align::Center, 55, 65);
            let text = CString::new("Dir").unwrap();
            label.set_text(text.as_c_str()).unwrap();

            // city - wind direction
            let mut wind_dir = Label::create(&mut city_pane).unwrap();
            wind_dir.set_align(Align::Center, 55, 80);
            let text = CString::new("").unwrap();
            wind_dir.set_text(text.as_c_str()).unwrap();
            wind_dir.add_style(Part::Main, Box::leak(font_12_blu_style()));

            // city - local weather forecast title label
            label = Label::create(&mut city_pane).unwrap();
            label.set_align(Align::Center, 0, 105);
            let text = CString::new("Local Weather Forecast").unwrap();
            label.set_text(text.as_c_str()).unwrap();

            // city forecast day 1 name title label
            let mut title_forecast_day_1 = Label::create(&mut city_pane).unwrap();
            title_forecast_day_1.set_align(Align::Center, -60, 120);
            let text = CString::new("").unwrap();
            title_forecast_day_1.set_text(text.as_c_str()).unwrap();
            title_forecast_day_1.add_style(Part::Main, Box::leak(font_12_red_style()));

            // city forecast day 1
            let mut forecast_day_1 = Label::create(&mut city_pane).unwrap();
            forecast_day_1.set_align(Align::Center, -60, 145);
            let text = CString::new("").unwrap();
            forecast_day_1.set_text(text.as_c_str()).unwrap();
            forecast_day_1.add_style(Part::Main, Box::leak(font_12_blu_style()));

            // city forecast day 2 name title label
            let mut title_forecast_day_2 = Label::create(&mut city_pane).unwrap();
            title_forecast_day_2.set_align(Align::Center, 0, 120);
            let text = CString::new("").unwrap();
            title_forecast_day_2.set_text(text.as_c_str()).unwrap();
            title_forecast_day_2.add_style(Part::Main, Box::leak(font_12_red_style()));

            // city forecast day 2
            let mut forecast_day_2 = Label::create(&mut city_pane).unwrap();
            forecast_day_2.set_align(Align::Center, 0, 145);
            let text = CString::new("").unwrap();
            forecast_day_2.set_text(text.as_c_str()).unwrap();
            forecast_day_2.add_style(Part::Main, Box::leak(font_12_blu_style()));

            // city forecast day 3 name title label
            let mut title_forecast_day_3 = Label::create(&mut city_pane).unwrap();
            title_forecast_day_3.set_align(Align::Center, 60, 120);
            let text = CString::new("").unwrap();
            title_forecast_day_3.set_text(text.as_c_str()).unwrap();
            title_forecast_day_3.add_style(Part::Main, Box::leak(font_12_red_style()));

            // city forecast day 3
            let mut forecast_day_3 = Label::create(&mut city_pane).unwrap();
            forecast_day_3.set_align(Align::Center, 60, 145);
            let text = CString::new("").unwrap();
            forecast_day_3.set_text(text.as_c_str()).unwrap();
            forecast_day_3.add_style(Part::Main, Box::leak(font_12_blu_style()));

            // city last updated
            let mut last_update = Label::create(&mut city_pane).unwrap();
            last_update.set_align(Align::Center, 0, 175);
            let text = CString::new("").unwrap();
            last_update.set_text(text.as_c_str()).unwrap();
            last_update.add_style(Part::Main, Box::leak(font_10_blk_style()));

            cities_labels.push(CityLabels {
                title,
                time,
                date,
                temp,
                weather_descr,
                uv,
                feels_like,
                aqi,
                wind_speed,
                wind_gust,
                wind_dir,
                title_forecast_day_1,
                forecast_day_1,
                title_forecast_day_2,
                forecast_day_2,
                title_forecast_day_3,
                forecast_day_3,
                last_update,
            })
        }

        Self {
            home_pane,
            cities_labels,
        }
    }

    pub fn show(&mut self) {
        lvgl_misc::show_obj(&mut self.home_pane);
    }

    pub fn hide(&mut self) {
        lvgl_misc::hide_obj(&mut self.home_pane);
    }

    pub fn set_cities_title(&mut self, cities_info: Vec<CityInfo>) {
        for (i, city) in cities_info.iter().enumerate() {
            let city_title = format!("{}\n{} {}", city.city_name, city.state, city.zipcode);

            self.cities_labels[i]
                .title
                .set_text(CString::new(city_title.as_str()).unwrap().as_c_str())
                .unwrap();
        }
    }

    pub fn set_city_time_date(&mut self, city_number: usize, time: String, date: String) {
        self.cities_labels[city_number]
            .time
            .set_text(CString::new(time.as_str()).unwrap().as_c_str())
            .unwrap();

        self.cities_labels[city_number]
            .date
            .set_text(CString::new(date.as_str()).unwrap().as_c_str())
            .unwrap();
    }

    pub fn set_city_forecast(&mut self, city_number: usize, city_forecast: CityForecast) {
        // Set city temperature
        self.cities_labels[city_number]
            .temp
            .set_text(
                CString::new(city_forecast.temp.as_str())
                    .unwrap()
                    .as_c_str(),
            )
            .unwrap();

        // Set city weather description
        self.cities_labels[city_number]
            .weather_descr
            .set_text(
                CString::new(city_forecast.weather_descr.as_str())
                    .unwrap()
                    .as_c_str(),
            )
            .unwrap();

        // Set city feels like
        self.cities_labels[city_number]
            .feels_like
            .set_text(
                CString::new(city_forecast.feels_like.as_str())
                    .unwrap()
                    .as_c_str(),
            )
            .unwrap();

        // Set city UV value and background color
        self.cities_labels[city_number]
            .uv
            .set_text(CString::new(city_forecast.uv.as_str()).unwrap().as_c_str())
            .unwrap();

        let color = get_uv_bg_color(city_forecast.uv.as_str().parse::<u8>().unwrap());
        lvgl_misc::set_label_bg_color(&mut self.cities_labels[city_number].uv, color);

        // Set city air quality index and background color
        self.cities_labels[city_number]
            .aqi
            .set_text(CString::new(city_forecast.aqi.as_str()).unwrap().as_c_str())
            .unwrap();

        let color = get_aqi_bg_color(city_forecast.aqi.as_str().parse::<u16>().unwrap());
        lvgl_misc::set_label_bg_color(&mut self.cities_labels[city_number].aqi, color);

        // Set city wind speed
        self.cities_labels[city_number]
            .wind_speed
            .set_text(
                CString::new(city_forecast.wind_speed.as_str())
                    .unwrap()
                    .as_c_str(),
            )
            .unwrap();

        // Set city wind gust
        self.cities_labels[city_number]
            .wind_gust
            .set_text(
                CString::new(city_forecast.wind_gust.as_str())
                    .unwrap()
                    .as_c_str(),
            )
            .unwrap();

        // Set city wind direction
        self.cities_labels[city_number]
            .wind_dir
            .set_text(
                CString::new(city_forecast.wind_dir.as_str())
                    .unwrap()
                    .as_c_str(),
            )
            .unwrap();

        // Set city day 1 weekday title
        self.cities_labels[city_number]
            .title_forecast_day_1
            .set_text(
                CString::new(city_forecast.weekday_forecast_day_1.as_str())
                    .unwrap()
                    .as_c_str(),
            )
            .unwrap();

        // Set city day 1 forecast
        self.cities_labels[city_number]
            .forecast_day_1
            .set_text(
                CString::new(city_forecast.forecast_day_1.as_str())
                    .unwrap()
                    .as_c_str(),
            )
            .unwrap();

        // Set city day 2 weekday title
        self.cities_labels[city_number]
            .title_forecast_day_2
            .set_text(
                CString::new(city_forecast.weekday_forecast_day_2.as_str())
                    .unwrap()
                    .as_c_str(),
            )
            .unwrap();

        // Set city day 2 forecast
        self.cities_labels[city_number]
            .forecast_day_2
            .set_text(
                CString::new(city_forecast.forecast_day_2.as_str())
                    .unwrap()
                    .as_c_str(),
            )
            .unwrap();

        // Set city day 3 weekday title
        self.cities_labels[city_number]
            .title_forecast_day_3
            .set_text(
                CString::new(city_forecast.weekday_forecast_day_3.as_str())
                    .unwrap()
                    .as_c_str(),
            )
            .unwrap();

        // Set city day 3 forecast
        self.cities_labels[city_number]
            .forecast_day_3
            .set_text(
                CString::new(city_forecast.forecast_day_3.as_str())
                    .unwrap()
                    .as_c_str(),
            )
            .unwrap();

        // Set city last update
        self.cities_labels[city_number]
            .last_update
            .set_text(
                CString::new(city_forecast.last_update.as_str())
                    .unwrap()
                    .as_c_str(),
            )
            .unwrap();
    }
}

fn get_uv_bg_color(uv_value: u8) -> Color {
    match uv_value {
        // 1-2 means UV is Low
        0..=2 => Color::from_rgb((0, 128, 0)), // green
        // 3-5 means UV is Moderate
        3..=5 => Color::from_rgb((255, 255, 0)), // yellow 1
        // 6-7 means UV is High
        6..=7 => Color::from_rgb((255, 140, 0)), // dark orange
        // 8-10 means UV is Very High
        8..=10 => Color::from_rgb((255, 0, 0)), // red 1
        // 11 and greater means UV is Extremely High
        11..=u8::MAX => Color::from_rgb((153, 50, 204)), // dark orchid
    }
}

fn get_aqi_bg_color(aqi_value: u16) -> Color {
    match aqi_value {
        // 1 means Good
        1 => Color::from_rgb((0, 128, 0)), // green
        // 2 means Moderate
        2 => Color::from_rgb((255, 255, 0)), // yellow 1
        // 3 means Unhealthy for sensitive group
        3 => Color::from_rgb((255, 140, 0)), // dark orange
        // 4 means Unhealthy
        4 => Color::from_rgb((255, 0, 0)), // red 1
        // 5 means Very Unhealthy
        5 => Color::from_rgb((153, 50, 204)), // dark orchid
        // 6 means Hazardous
        6 => Color::from_rgb((139, 28, 98)), // maroon4
        // should never get here
        _ => Color::from_rgb((255, 255, 255)), // white
    }
}

//*****************************************************************************
//                      Sytles for Home Pane
//*****************************************************************************
fn home_pane_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_bg_color(Color::from_rgb((0, 0, 0)));
    style.set_radius(0);
    style.set_border_width(0);
    style.set_pad_top(12);
    style.set_pad_bottom(12);
    style.set_pad_left(12);
    style.set_pad_right(12);
    style.set_layout(Layout::flex());
    style.set_flex_flow(FlexFlow::ROW);
    style.set_flex_main_place(FlexAlign::CENTER);

    Box::new(style)
}

fn city_pane_style() -> Box<Style> {
    let mut style = Style::default();
    // light steel blue 176.196,222 Antique White 250, 235, 215 yellow 2 238, 238, 0
    style.set_bg_color(Color::from_rgb((255, 215, 0))); // gold
    style.set_border_width(0);
    style.set_pad_top(2);
    style.set_pad_bottom(2);
    style.set_pad_left(2);
    style.set_pad_right(2);

    Box::new(style)
}

// Font 16 color red
fn city_title_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_text_color(Color::from_rgb((139, 0, 0))); // red4
    style.set_text_align(TextAlign::Center);
    style.set_text_font(unsafe { Font::new_raw(lv_font_montserrat_16) });

    Box::new(style)
}

// Font 24 color black
fn time_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_text_color(Color::from_rgb((0, 0, 0))); // black
    style.set_text_align(TextAlign::Center);
    style.set_text_font(unsafe { Font::new_raw(lv_font_montserrat_24) });

    Box::new(style)
}

// Color scale, black font, green background
fn color_scale_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_text_color(Color::from_rgb((0, 0, 0))); // black
    style.set_text_align(TextAlign::Center);
    style.set_text_font(unsafe { Font::new_raw(lv_font_montserrat_16) });
    style.set_border_color(Color::from_rgb((0, 0, 0))); // black
    style.set_border_width(2);
    style.set_bg_opa(Opacity::OPA_COVER);

    Box::new(style)
}

// Font 36 color blue
fn temperature_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_text_color(Color::from_rgb((0, 0, 139))); // navy blue
    style.set_text_align(TextAlign::Center);
    style.set_text_font(unsafe { Font::new_raw(lv_font_montserrat_36) });

    Box::new(style)
}

// Font 14 color blue
fn font_14_blu_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_text_color(Color::from_rgb((0, 0, 139))); // navy blue
    style.set_text_align(TextAlign::Center);
    style.set_text_font(unsafe { Font::new_raw(lv_font_montserrat_16) });

    Box::new(style)
}

// Font 12 color blue
fn font_12_blu_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_text_color(Color::from_rgb((0, 0, 139))); // navy blue
    style.set_text_align(TextAlign::Center);
    style.set_text_font(unsafe { Font::new_raw(lv_font_montserrat_12) });

    Box::new(style)
}

// Font 12 color red
fn font_12_red_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_text_color(Color::from_rgb((139, 0, 0))); // red4
    style.set_text_align(TextAlign::Center);
    style.set_text_font(unsafe { Font::new_raw(lv_font_montserrat_12) });

    Box::new(style)
}

// Font 10 color black
fn font_10_blk_style() -> Box<Style> {
    let mut style = Style::default();
    style.set_text_color(Color::from_rgb((0, 0, 0))); // black
    style.set_text_align(TextAlign::Center);
    style.set_text_font(unsafe { Font::new_raw(lv_font_montserrat_10) });

    Box::new(style)
}
