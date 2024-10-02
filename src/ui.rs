use crate::cities_settings_pane::CitiesSettingsPane;
use crate::gt911::{TouchState, GT911};
use crate::home_pane::HomePane;
use crate::lcd_panel::{LcdPanel, PanelConfig, PanelFlagsConfig, TimingFlagsConfig, TimingsConfig};
use crate::model::ModelRequest;
use crate::model::{CityForecast, CityInfo};
use crate::navigation_pane::NavigationPane;
use crate::startup_pane::StartupPane;
use crate::wifi_settings_pane::WifiSettingsPane;

use std::cell::RefCell;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use log::info;
//use log::warn;

use embedded_graphics_core::prelude::Point;
use embedded_hal::i2c::I2c;
use esp_idf_svc::hal::delay::FreeRtos;

use lvgl::input_device::{
    pointer::{Pointer, PointerInputData},
    InputDriver,
};

use lvgl::{Display, DrawBuffer};

#[derive(Debug)]
pub enum UiRequest {
    ShowStartup,
    ShowWifiSettings,
    ShowHome,
    SetStartupMessage(String),
    SetWifiSettingsErrorMessage(String),
    SetCityTime(usize, String, String),
    SetCitiesSettings(Vec<CityInfo>),
    SetCitiesTitles(Vec<CityInfo>),
    SetCityForecast(usize, CityForecast),
    SetWifiCreds(String, String),
}

pub struct UserInterface<I2C>
where
    I2C: I2c,
{
    gt911: GT911<I2C>,
    tx: Sender<ModelRequest>,
    rx: Receiver<UiRequest>,
}

impl<I2C> UserInterface<I2C>
where
    I2C: I2c + Send + 'static,
{
    pub fn new(gt911: GT911<I2C>, tx: Sender<ModelRequest>, rx: Receiver<UiRequest>) -> Self {
        Self { gt911, tx, rx }
    }

    pub fn run(self) {
        info!("---------- Creating UI Thread ----------");
        let _lvgl_thread = thread::Builder::new().stack_size(24 * 1024).spawn(move || {
            const HOR_RES: u32 = 800;
            const VER_RES: u32 = 480;
            const LINES: u32 = 4; // The number of lines (rows) that will be refreshed
            const DRAW_BUFFER_SIZE: usize = (HOR_RES * LINES) as usize;

            lvgl::init();

            let mut lcd_panel = LcdPanel::new(
                &PanelConfig::new(),
                &PanelFlagsConfig::new(),
                &TimingsConfig::new(),
                &TimingFlagsConfig::new(),
            )
            .unwrap();

            let draw_buffer = DrawBuffer::<{ DRAW_BUFFER_SIZE }>::default();
            let display = Display::register(draw_buffer, HOR_RES, VER_RES, |refresh| {
                lcd_panel
                    .set_pixels_lvgl_color(
                        refresh.area.x1.into(),
                        refresh.area.y1.into(),
                        (refresh.area.x2 + 1i16).into(),
                        (refresh.area.y2 + 1i16).into(),
                        refresh.colors,
                    )
                    .unwrap();
            })
            .unwrap();

            let touch_screen = RefCell::new(self.gt911);
            let read_touchscreen_cb = || {
                let touch = touch_screen.borrow_mut().read_touch().unwrap();

                match touch {
                    TouchState::PRESSED(tp) => {
                        //info!("Pressed");
                        PointerInputData::Touch(Point {
                            x: tp.x as i32,
                            y: tp.y as i32,
                        })
                        .pressed()
                        .once()
                    }

                    TouchState::RELEASED(tp) => {
                        //info!("Released");
                        PointerInputData::Touch(Point {
                            x: tp.x as i32,
                            y: tp.y as i32,
                        })
                        .released()
                        .once()
                    }
                }
            };

            // Register the touchscreen callback with the display
            let _the_touch_screen = Pointer::register(read_touchscreen_cb, &display).unwrap();

            let mut startup_pane_parent = display.get_scr_act().unwrap();
            let mut startup_pane = StartupPane::new(&mut startup_pane_parent);
            startup_pane.hide();

            let mut navigation_pane_parent = display.get_scr_act().unwrap();
            let mut nav_pane = NavigationPane::new(&mut navigation_pane_parent);
            nav_pane.hide();

            let mut home_pane_parent = display.get_scr_act().unwrap();
            let mut home_pane = HomePane::new(&mut home_pane_parent);
            home_pane.hide();

            let mut wifi_settings_pane_parent = display.get_scr_act().unwrap();
            let mut ws_pane = WifiSettingsPane::new(&mut wifi_settings_pane_parent);
            ws_pane.hide();

            let mut cities_settings_pane_parent = display.get_scr_act().unwrap();
            let mut cs_pane = CitiesSettingsPane::new(&mut cities_settings_pane_parent);
            cs_pane.hide();

            // Textarea events - assign the virtual keyboard to this textarea
            ws_pane
                .ssid_ta
                .on_event(|mut ta, event| {
                    if event == lvgl::Event::Clicked {
                        ws_pane.keyboard.set_textarea(&mut ta);
                    }
                })
                .unwrap();

            ws_pane
                .pswd_ta
                .on_event(|mut ta, event| {
                    if event == lvgl::Event::Clicked {
                        ws_pane.keyboard.set_textarea(&mut ta);
                    }
                })
                .unwrap();

            const CITY_1: usize = 0;
            const CITY_2: usize = 1;
            const CITY_3: usize = 2;
            const CITY_4: usize = 3;

            cs_pane.cities_widgets[CITY_1]
                .city_name
                .on_event(|mut ta, event| {
                    if event == lvgl::Event::Clicked {
                        cs_pane.keyboard.set_textarea(&mut ta);
                    }
                })
                .unwrap();

            cs_pane.cities_widgets[CITY_1]
                .zipcode
                .on_event(|mut ta, event| {
                    if event == lvgl::Event::Clicked {
                        cs_pane.keyboard.set_textarea(&mut ta);
                    }
                })
                .unwrap();

            cs_pane.cities_widgets[CITY_2]
                .city_name
                .on_event(|mut ta, event| {
                    if event == lvgl::Event::Clicked {
                        cs_pane.keyboard.set_textarea(&mut ta);
                    }
                })
                .unwrap();

            cs_pane.cities_widgets[CITY_2]
                .zipcode
                .on_event(|mut ta, event| {
                    if event == lvgl::Event::Clicked {
                        cs_pane.keyboard.set_textarea(&mut ta);
                    }
                })
                .unwrap();

            cs_pane.cities_widgets[CITY_3]
                .city_name
                .on_event(|mut ta, event| {
                    if event == lvgl::Event::Clicked {
                        cs_pane.keyboard.set_textarea(&mut ta);
                    }
                })
                .unwrap();

            cs_pane.cities_widgets[CITY_3]
                .zipcode
                .on_event(|mut ta, event| {
                    if event == lvgl::Event::Clicked {
                        cs_pane.keyboard.set_textarea(&mut ta);
                    }
                })
                .unwrap();

            cs_pane.cities_widgets[CITY_4]
                .city_name
                .on_event(|mut ta, event| {
                    if event == lvgl::Event::Clicked {
                        cs_pane.keyboard.set_textarea(&mut ta);
                    }
                })
                .unwrap();

            cs_pane.cities_widgets[CITY_4]
                .zipcode
                .on_event(|mut ta, event| {
                    if event == lvgl::Event::Clicked {
                        cs_pane.keyboard.set_textarea(&mut ta);
                    }
                })
                .unwrap();

            // Button clicked events - set the appropriate flag that will be checked inside the loop
            let mut ws_pane_edit_btn_clicked = false;
            ws_pane
                .edit_btn
                .on_event(|_btn, event| {
                    if let lvgl::Event::Clicked = event {
                        ws_pane_edit_btn_clicked = true;
                    }
                })
                .unwrap();

            let mut ws_pane_save_btn_clicked = false;
            ws_pane
                .save_btn
                .on_event(|_btn, event| {
                    if let lvgl::Event::Clicked = event {
                        ws_pane_save_btn_clicked = true;
                    }
                })
                .unwrap();

            let mut ws_pane_exit_btn_clicked = false;
            ws_pane
                .exit_btn
                .on_event(|_btn, event| {
                    if let lvgl::Event::Clicked = event {
                        ws_pane_exit_btn_clicked = true;
                    }
                })
                .unwrap();

            let mut cs_pane_edit_btn_clicked = false;
            cs_pane
                .edit_btn
                .on_event(|_btn, event| {
                    if let lvgl::Event::Clicked = event {
                        cs_pane_edit_btn_clicked = true;
                    }
                })
                .unwrap();

            let mut cs_pane_save_btn_clicked = false;
            cs_pane
                .save_btn
                .on_event(|_btn, event| {
                    if let lvgl::Event::Clicked = event {
                        cs_pane_save_btn_clicked = true;
                    }
                })
                .unwrap();

            let mut cs_pane_exit_btn_clicked = false;
            cs_pane
                .exit_btn
                .on_event(|_btn, event| {
                    if let lvgl::Event::Clicked = event {
                        cs_pane_exit_btn_clicked = true;
                    }
                })
                .unwrap();

            let mut nav_pane_btn_matrix_btn_clicked = false;
            nav_pane
                .btn_matrix
                .on_event(|_btnm, event| {
                    if let lvgl::Event::Pressed = event {
                        nav_pane_btn_matrix_btn_clicked = true;
                    }
                })
                .unwrap();

            const BTN_ID_HOME: u16 = 0;
            const BTN_ID_WIFI_SETTINGS: u16 = 1;
            const BTN_ID_CITIES_SETTINGS: u16 = 2;

            info!("Entering UI thread loop");

            // The loop - do not exit thread and run forever in this thread
            loop {
                if let Ok(gui_request) = self.rx.try_recv() {
                    match gui_request {
                        UiRequest::ShowStartup => {
                            ws_pane.hide();
                            nav_pane.hide();
                            home_pane.hide();
                            startup_pane.show();
                        }

                        UiRequest::SetStartupMessage(msg) => {
                            startup_pane.set_message(msg);
                        }

                        UiRequest::ShowHome => {
                            startup_pane.hide();
                            nav_pane.show();
                            nav_pane.set_new_btn_selected(BTN_ID_HOME);
                            home_pane.show();
                        }

                        UiRequest::ShowWifiSettings => {
                            startup_pane.hide();
                            nav_pane.set_new_btn_selected(BTN_ID_WIFI_SETTINGS);
                            nav_pane.show();
                            ws_pane.show();
                        }

                        UiRequest::SetWifiSettingsErrorMessage(msg) => {
                            ws_pane.set_error_message(msg);
                        }

                        UiRequest::SetCityTime(city_number, time, date) => {
                            home_pane.set_city_time_date(city_number, time, date);
                        }

                        UiRequest::SetCitiesTitles(cities_info) => {
                            home_pane.set_cities_title(cities_info);
                        }

                        UiRequest::SetCitiesSettings(cities_info) => {
                            cs_pane.set_cities_settings(cities_info);
                        }

                        UiRequest::SetCityForecast(city_number, city_forecast) => {
                            home_pane.set_city_forecast(city_number, city_forecast);
                        }

                        UiRequest::SetWifiCreds(ssid, pass) => {
                            ws_pane.set_wifi_credentials(ssid, pass);
                        }
                    }
                }
                // Check button clicked flags
                if nav_pane_btn_matrix_btn_clicked {
                    let btn_id = nav_pane.get_btn_selected();
                    nav_pane.set_new_btn_selected(btn_id);

                    match btn_id {
                        // Home button clicked
                        BTN_ID_HOME => {
                            cs_pane.hide();
                            ws_pane.hide();
                            home_pane.show();
                        }
                        // Wifi settings button clicked
                        BTN_ID_WIFI_SETTINGS => {
                            home_pane.hide();
                            cs_pane.hide();
                            ws_pane.show();
                        }
                        // Cities settings button clicked
                        BTN_ID_CITIES_SETTINGS => {
                            home_pane.hide();
                            ws_pane.hide();
                            cs_pane.show();
                        }
                        _ => {}
                    }
                    nav_pane_btn_matrix_btn_clicked = false;
                }

                if cs_pane_edit_btn_clicked {
                    cs_pane_edit_btn_clicked = false;
                    cs_pane.enable_editing();
                }

                if cs_pane_save_btn_clicked {
                    cs_pane_save_btn_clicked = false;

                    if let Some(user_cities_info) = cs_pane.get_user_cities_settings() {
                        send_cities_info_update(&self.tx, user_cities_info);
                        cs_pane.hide();
                    }
                }

                if cs_pane_exit_btn_clicked {
                    cs_pane_exit_btn_clicked = false;
                    nav_pane.set_home_btn_selected();
                    cs_pane.hide();
                    home_pane.show();
                }

                if ws_pane_edit_btn_clicked {
                    ws_pane_edit_btn_clicked = false;
                    ws_pane.enable_editing();
                }

                if ws_pane_save_btn_clicked {
                    ws_pane_save_btn_clicked = false;
                    let (ssid, pass) = ws_pane.get_user_wifi_creds_entries();
                    send_wifi_credentials_update(&self.tx, ssid.clone(), pass.clone());
                    ws_pane.hide();
                }

                if ws_pane_exit_btn_clicked {
                    ws_pane_exit_btn_clicked = false;
                    nav_pane.set_home_btn_selected();
                    ws_pane.hide();
                    home_pane.show();
                }

                lvgl::task_handler();

                // Give other threads chance to run
                FreeRtos::delay_ms(10);
            } // end loop
        });
    }
}

fn send_wifi_credentials_update(tx: &Sender<ModelRequest>, ssid: String, pass: String) {
    tx.send(ModelRequest::UpdateWifiCreds(ssid, pass)).unwrap();
}

fn send_cities_info_update(tx: &Sender<ModelRequest>, cities_info: Vec<CityInfo>) {
    tx.send(ModelRequest::UpdateCitiesInfo(cities_info.clone()))
        .unwrap();
}
